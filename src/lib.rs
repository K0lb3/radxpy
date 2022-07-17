use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyBytes;

use std::error::Error;
use std::io::{BufReader, BufWriter, Cursor, Read};

use hound::{WavSpec, SampleFormat, WavWriter, WavReader, Error as WavError, Result as WavResult};
use radx::{AdxSpec, LoopInfo};
use radx::encoder::standard_encoder::StandardEncoder;
use radx::encoder::ahx_encoder::AhxEncoder;

#[pymodule]
fn radxpy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    Ok(())
}

// use proc_macro;
// #[proc_macro]
// pub fn barf_py(message: &str) -> ! {
//     panic!(message);
// }

fn barf(message: &str) -> ! {
    panic!("Error: {}", message);
    //process::exit(1);
}

fn unwrap_or_barf<T, E>(result: Result<T, E>, err_desc: &str) -> T
    where E: Error
{
    result.unwrap_or_else(|err| {
        let err_string = format!("{}: {}", err_desc, err);
        barf(&err_string);
    })
}

/*******************************************************************

                DECODE

*******************************************************************/

#[pyfunction]
pub fn decode(py: Python, adx_data: Vec<u8>, loops: u32) -> PyObject {
    // Open adx file and make reader/print header
    let adx_data_len = adx_data.len();
    let adx_file = BufReader::new(Cursor::new(adx_data));
    let mut adx = unwrap_or_barf(radx::from_reader(adx_file, loops > 0), "Could not make adx reader");

    // Make wav spec
    let spec = WavSpec {
        channels: adx.channels() as u16,
        sample_rate: adx.sample_rate(),
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    // Open wav writer
    // using a very rough estimate of the size of the wav data
    // actual size: 44 + adx.channels() + adx.sample_rate() + adx.samples.len() * 2
    // adx.samples is sadly private
    let mut buf = Vec::with_capacity(adx_data_len*4);
    let wav_file = BufWriter::new(Cursor::new(&mut buf));
    let mut wav_writer = unwrap_or_barf(WavWriter::new(wav_file, spec), "Could not make wav writer");

    // Read depending on number of loops
    if loops > 0 {
        if let Some(loop_info) = adx.loop_info() {
            let samples_to_read = loop_info.start_sample + loops * (loop_info.end_sample - loop_info.start_sample);
            for _ in 0..samples_to_read {
                let sample = adx.next_sample().unwrap();
                for channel_sample in sample {
                    unwrap_or_barf(wav_writer.write_sample(channel_sample), "Problem writing wav samples");
                }
            }
        }
        else {
            barf("File is not a looping ADX. Do not use \"-l\"");
        }
    }
    else {
        for sample in adx {
            for channel_sample in sample {
                unwrap_or_barf(wav_writer.write_sample(channel_sample), "Problem writing wav samples");
            }
        }
    };

    // Finish writing to the wav
    unwrap_or_barf(wav_writer.finalize(), "Could not finalize writing wav file");

    // Finish writing to the wav
    PyBytes::new(py, &buf).into()
}

/*******************************************************************

                ENCODE

*******************************************************************/

#[pyfunction]
pub fn encode(py: Python, wav_data: Vec<u8>, start: u32, end: u32, no_loop: bool, ahx: bool) -> PyObject {
    // Open input and output files
    let input = BufReader::new(Cursor::new(wav_data));
    let mut buf = Vec::new();
    let output = BufWriter::new(Cursor::new(&mut buf));

    // Change based on encoding
    if ahx {
        // Read samples
        println!("Reading Samples");
        let (samples, sample_rate) = unwrap_or_barf(read_samples_ahx(input), "Could not read samples from input");

        if sample_rate != 22050 {
            barf("ahx encoding requires a sample rate of 22050");
        }

        // Make encoder
        let mut encoder = unwrap_or_barf(AhxEncoder::new(output), "Could not make encoder");

        // Encode data
        println!("Encoding data");
        unwrap_or_barf(encoder.encode_data(samples), "Could not encode data");
        unwrap_or_barf(encoder.finalize(), "Could not finish writing adx file");
    }
    else {
        // Read samples
        println!("Reading Samples");
        let (samples, sample_rate) = unwrap_or_barf(read_samples(input), "Could not read samples from input");

        // Make adx spec
        let spec = if no_loop {
            AdxSpec {
                channels: 2,
                sample_rate: sample_rate,
                loop_info: None,
            }
        }
        else {
            AdxSpec {
                channels: 2,
                sample_rate: sample_rate,
                loop_info: Some(
                    LoopInfo {
                        start_sample: start,
                        end_sample: if end>0 {end} else {samples.len() as u32},
                    }
                )
            }
        };

        // Make encoder from spec
        let mut encoder = unwrap_or_barf(StandardEncoder::new(output, spec), "Could not make encoder");

        // Encode data
        println!("Encoding data");
        unwrap_or_barf(encoder.encode_data(samples), "Could not encode data");
        unwrap_or_barf(encoder.finish(), "Could not finish writing adx file");
    }

    // Finish writing to the wav
    PyBytes::new(py, &buf).into()
}

fn read_samples<R>(reader: R) -> WavResult<(Vec<Vec<i16>>, u32)>
    where R: Read
{
    let mut reader = WavReader::new(reader)?;
    let spec = reader.spec();
    if spec.channels == 1 {
        let mut samples = reader.samples::<i16>();
        let mut sample_vec = Vec::new();
        while let Some(sample_res) = samples.next() {
            let sample = sample_res?;
            sample_vec.push(vec![sample, sample]);
        }
        Ok((sample_vec, spec.sample_rate))
    }
    else if spec.channels == 2 {
        let mut samples = reader.samples::<i16>();
        let mut sample_vec = Vec::new();
        while let Some(sample1_res) = samples.next() {
            let sample1 = sample1_res?;
            let sample2 = if let Some(sample_res) = samples.next() {
                sample_res?
            }
            else {
                sample1
            };
            sample_vec.push(vec![sample1, sample2]);
        }
        Ok((sample_vec, spec.sample_rate))
    }
    else {
        Err(WavError::Unsupported)
    }
}

fn read_samples_ahx<R>(reader: R) -> WavResult<(Vec<i16>, u32)>
    where R: Read
{
    let mut reader = WavReader::new(reader)?;
    let spec = reader.spec();
    if spec.channels == 1 {
        let samples: WavResult<_> = reader.samples::<i16>().collect();
        Ok((samples?, spec.sample_rate))
    }
    else {
        barf("ahx encoding requires 1 channel (mono)");
    }
}
