// Copyright (c) Bao Project and Contributors. All rights reserved.
//          João Peixoto <joaopeixotooficial@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

//! Bao utility functions.

#![allow(dead_code)]

use super::types::*;
use clap::{App, Arg};
use std::env;
use std::fs::File;
use std::io::Read;

/// Represents a collection of ParamKey.
///
/// # Attributes
///
/// * `VmId` - Frontend ID
/// * `DevId` - Device Compatibility ID (e.g. 22 for I2C)
/// * `DevIrq` - Device IRQ
/// * `DevAddr` - Device Address
/// * `RamAddr` - RAM Address
/// * `RamSize` - RAM Size
#[derive(Clone)]
enum ParamKey {
    VmId = 0,
    DevId,
    DevIrq,
    DevAddr,
    RamAddr,
    RamSize,
}

/// Function to transpose a matrix.
///
/// # Arguments
///
/// * `matrix` - A reference to a matrix.
///
/// # Returns
///
/// * `Vec<Vec<u64>>` - A transposed matrix.
fn transpose_matrix(matrix: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut transposed = Vec::new();

    if !matrix.is_empty() {
        let rows = matrix.len();
        let cols = matrix[0].len();

        for j in 0..cols {
            let mut column = Vec::new();
            for i in 0..rows {
                column.push(matrix[i][j]);
            }
            transposed.push(column);
        }
    }

    transposed
}

/// Parses the command line arguments.
///
/// # Returns
///
/// * `Option<Vec<Vec<u64>>>` - A vector of tuples containing the parameters.
///
/// # Examples
///
/// $ bao-vhost-frontend vm_id=0 dev_id=22 dev_irq=47 dev_addr=167788032 ram_addr=1476395008 ram_size=16777216
///
/// $ bao-vhost-frontend vm_id=0,1 dev_id=22,29 dev_irq=47,46 dev_addr=167788032,167787520 ram_addr=1476395008,1493172224 ram_size=16777216,16777216
pub fn parse_command_line_arguments() -> Option<Vec<Vec<u64>>> {
    // Initialize the parameters
    let mut parameters: Vec<Vec<u64>> = Vec::new();

    // Get the environment command line arguments
    let args = env::args().collect::<Vec<String>>();

    // Pop the first argument (executable name)
    let args = args[1..].to_vec();

    // Parse the parameters string
    for arg in args.iter() {
        // Split the parameter into key and value
        let parts: Vec<&str> = arg.split('=').collect();
        if parts.len() != 2 {
            return None; // Invalid format
        }

        // Update the key
        let key = match parts[0] {
            "vm_id" => ParamKey::VmId,
            "dev_id" => ParamKey::DevId,
            "dev_irq" => ParamKey::DevIrq,
            "dev_addr" => ParamKey::DevAddr,
            "ram_addr" => ParamKey::RamAddr,
            "ram_size" => ParamKey::RamSize,
            _ => return None, // Unknown key
        };

        // Update the value
        let value: Option<Vec<u64>> = match key {
            ParamKey::VmId
            | ParamKey::DevId
            | ParamKey::DevIrq
            | ParamKey::DevAddr
            | ParamKey::RamAddr
            | ParamKey::RamSize => {
                // Split the value into parts
                let value_parts: Vec<u64> =
                    parts[1].split(',').filter_map(|s| s.parse().ok()).collect();
                // Check if the value is empty
                if value_parts.is_empty() {
                    return None; // Invalid range format
                }
                // Return the value
                Some(value_parts)
            }
        };

        // Clone the key and check if the index > length
        let key_index = key.clone() as usize;
        if key_index > parameters.len() {
            return None;
        }
        // Update the corresponding parameter
        parameters.insert(key as usize, value.unwrap().clone());
    }

    // Check if all parameters are present and with the same length
    if parameters.len() != 6
        || parameters[ParamKey::VmId as usize].is_empty()
        || parameters[ParamKey::DevId as usize].is_empty()
        || parameters[ParamKey::DevIrq as usize].is_empty()
        || parameters[ParamKey::DevAddr as usize].is_empty()
        || parameters[ParamKey::RamAddr as usize].is_empty()
        || parameters[ParamKey::RamSize as usize].is_empty()
        || parameters[ParamKey::VmId as usize].len() != parameters[ParamKey::DevId as usize].len()
        || parameters[ParamKey::VmId as usize].len() != parameters[ParamKey::DevIrq as usize].len()
        || parameters[ParamKey::VmId as usize].len() != parameters[ParamKey::DevAddr as usize].len()
        || parameters[ParamKey::VmId as usize].len() != parameters[ParamKey::RamAddr as usize].len()
        || parameters[ParamKey::VmId as usize].len() != parameters[ParamKey::RamSize as usize].len()
    {
        return None;
    }

    // Transpose the matrix
    let transposed = transpose_matrix(&parameters);

    // Return the parameters
    Some(transposed)
}

/// Parses the YAML configuration file.
///
/// # Arguments
///
/// * `file_path` - A reference to a string containing the path to the YAML file.
///
/// # Returns
///
/// * `Result<ConfigFrontends, Box<dyn std::error::Error>>` - A ConfigFrontends struct containing the parsed configuration.
fn parse_yaml_config_file(file_path: &str) -> Result<ConfigFrontends, Box<dyn std::error::Error>> {
    // Open the YAML file
    let mut file = File::open(file_path).unwrap();
    // Read the YAML file
    let mut yaml_content = String::new();
    file.read_to_string(&mut yaml_content).unwrap();
    // Parse the YAML file
    let frontends: ConfigFrontends = serde_yaml::from_str(&yaml_content).unwrap();
    // Return the configuration
    Ok(frontends)
}

/// Parses the frontend arguments.
///
/// # Returns
///
/// * `Result<ConfigFrontends, Box<dyn std::error::Error>>` - A ConfigFrontends struct containing the parsed configuration.
///
/// # Examples
///
/// $ bao-vhost-frontend --config /path/to/your/config.yaml
///
/// or (short version)
///
/// $ bao-vhost-frontend -c /path/to/your/config.yaml
pub fn parse_arguments() -> Result<ConfigFrontends, Box<dyn std::error::Error>> {
    // Get the environment command line arguments
    let matches = App::new("Bao Vhost Frontend")
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // Extract the config file path
    let config_file = matches.value_of("config").unwrap();

    // Parse the YAML file
    let frontends = parse_yaml_config_file(config_file)?;

    // Return the configuration
    Ok(frontends)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parses the parameters string.
    ///
    /// # Arguments
    ///
    /// * `params` - A reference to a string containing the parameters.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<Vec<u64>>>` - A vector of tuples containing the parameters.
    fn parse_string_parameters(params: &str) -> Option<Vec<Vec<u64>>> {
        // Initialize the parameters
        let mut parameters: Vec<Vec<u64>> = Vec::new();

        // Parse the parameters string
        for param in params.split(',') {
            // Split the parameter into key and value
            let parts: Vec<&str> = param.split('=').collect();
            if parts.len() != 2 {
                return None; // Invalid format
            }

            // Update the key
            let key = match parts[0] {
                "vm_id" => ParamKey::VmId,
                "dev_id" => ParamKey::DevId,
                "dev_irq" => ParamKey::DevIrq,
                "dev_addr" => ParamKey::DevAddr,
                "ram_addr" => ParamKey::RamAddr,
                "ram_size" => ParamKey::RamSize,
                _ => return None, // Unknown key
            };

            // Update the value
            let value: Option<Vec<u64>> = match key {
                ParamKey::VmId
                | ParamKey::DevId
                | ParamKey::DevIrq
                | ParamKey::DevAddr
                | ParamKey::RamAddr
                | ParamKey::RamSize => {
                    // Split the value into parts
                    let value_parts: Vec<u64> =
                        parts[1].split('-').filter_map(|s| s.parse().ok()).collect();
                    // Check if the value is empty
                    if value_parts.is_empty() {
                        return None; // Invalid range format
                    }
                    // Return the value
                    Some(value_parts)
                }
            };

            // Clone the key and check if the index > length
            let key_index = key.clone() as usize;
            if key_index > parameters.len() {
                return None;
            }
            // Update the corresponding parameter
            parameters.insert(key as usize, value.unwrap().clone());
        }

        // Check if all parameters are present and with the same length
        if parameters.len() != 6
            || parameters[ParamKey::VmId as usize].is_empty()
            || parameters[ParamKey::DevId as usize].is_empty()
            || parameters[ParamKey::DevIrq as usize].is_empty()
            || parameters[ParamKey::DevAddr as usize].is_empty()
            || parameters[ParamKey::RamAddr as usize].is_empty()
            || parameters[ParamKey::RamSize as usize].is_empty()
            || parameters[ParamKey::VmId as usize].len()
                != parameters[ParamKey::DevId as usize].len()
            || parameters[ParamKey::VmId as usize].len()
                != parameters[ParamKey::DevIrq as usize].len()
            || parameters[ParamKey::VmId as usize].len()
                != parameters[ParamKey::DevAddr as usize].len()
            || parameters[ParamKey::VmId as usize].len()
                != parameters[ParamKey::RamAddr as usize].len()
            || parameters[ParamKey::VmId as usize].len()
                != parameters[ParamKey::RamSize as usize].len()
        {
            return None;
        }

        // Return the parameters
        Some(parameters)
    }

    #[test]
    fn test_parse_parameters_valid_single() {
        let params =
            "vm_id=0,dev_id=22,dev_irq=47,dev_addr=167788032,ram_addr=1476395008,ram_size=16777216";
        let parsed = parse_string_parameters(params);
        assert!(parsed.is_some());
        let parsed = parsed.unwrap();
        assert_eq!(parsed[ParamKey::VmId as usize], vec![0]);
        assert_eq!(parsed[ParamKey::DevId as usize], vec![22]);
        assert_eq!(parsed[ParamKey::DevIrq as usize], vec![47]);
        assert_eq!(parsed[ParamKey::DevAddr as usize], vec![167788032]);
        assert_eq!(parsed[ParamKey::RamAddr as usize], vec![1476395008]);
        assert_eq!(parsed[ParamKey::RamSize as usize], vec![16777216]);
    }

    #[test]
    fn test_parse_parameters_valid_multiple() {
        let params =
            "vm_id=0-1,dev_id=22-29,dev_irq=47-46,dev_addr=167788032-167787520,ram_addr=1476395008-1493172224,ram_size=16777216-16777216";
        let parsed = parse_string_parameters(params);
        assert!(parsed.is_some());
        let parsed = parsed.unwrap();
        assert_eq!(parsed[ParamKey::VmId as usize], vec![0, 1]);
        assert_eq!(parsed[ParamKey::DevId as usize], vec![22, 29]);
        assert_eq!(parsed[ParamKey::DevIrq as usize], vec![47, 46]);
        assert_eq!(
            parsed[ParamKey::DevAddr as usize],
            vec![167788032, 167787520]
        );
        assert_eq!(
            parsed[ParamKey::RamAddr as usize],
            vec![1476395008, 1493172224]
        );
        assert_eq!(parsed[ParamKey::RamSize as usize], vec![16777216, 16777216]);
    }

    #[test]
    fn test_parse_parameters_invalid_single() {
        // Not defined 'vm_id'
        let params =
            "dev_irq=47,dev_id=22,dev_addr=167788032,ram_addr=1476395008,ram_size=16777216";
        let parsed = parse_string_parameters(params);
        assert!(parsed.is_none());

        // Incorrect value for 'vm_id'
        let params =
            "vm_id=invalid,dev_id=22,dev_irq=47,dev_addr=167788032,ram_addr=1476395008,ram_size=16777216";
        let parsed = parse_string_parameters(params);
        assert!(parsed.is_none());

        // Invalid format
        let params = "vm_id=0,dev_id=22,dev_irq=47,dev_addr=167788032,ram_addr=1476395008,ram_size";
        let parsed = parse_string_parameters(params);
        assert!(parsed.is_none());

        // Out of order
        let params = "dev_id=22,dev_irq=47,dev_addr=167788032,,vm_id=0,ram_addr=1476395008,ram_size=16777216";
        let parsed = parse_string_parameters(params);
        assert!(parsed.is_none());
    }

    #[test]
    fn test_parse_parameters_invalid_multiple() {
        // Not defined 'ram_addr'
        let params =
            "vm_id=0-1,dev_id=22,dev_irq=47-48,dev_addr=167788032-167787520,ram_size=16777216-16777216";
        let parsed = parse_string_parameters(params);
        assert!(parsed.is_none());

        // Incorrect value for 'vm_id'
        let params =
            "vm_id=invalid-1,dev_id=22,dev_irq=47-48,dev_addr=167788032-167787520,ram_addr=1476395008-1493172224,ram_size=16777216-16777216";
        let parsed = parse_string_parameters(params);
        assert!(parsed.is_none());

        // Invalid format
        let params = "vm_id=0-1,dev_id=22-29,irq=47-48,dev_addr=167788032-167787520,ram_addr=1476395008,ram_size=16777216-16777216";
        let parsed = parse_string_parameters(params);
        assert!(parsed.is_none());

        // Out of order
        let params =
            "vm_id=0-1,dev_id=22-29,dev_irq=47-48,dev_addr=167788032-167787520,ram_size=16777216-16777216,ram_addr=1476395008-1493172224";
        let parsed = parse_string_parameters(params);
        assert!(parsed.is_none());
    }

    #[test]
    fn test_transpose() {
        let matrix: Vec<Vec<u64>> = vec![
            vec![0, 1],
            vec![22, 29],
            vec![47, 48],
            vec![167788032, 167787520],
            vec![1476395008, 1493172224],
            vec![16777216, 16777216],
        ];
        let transposed = transpose_matrix(&matrix);
        assert_eq!(
            transposed,
            vec![
                vec![0, 22, 47, 167788032, 1476395008, 16777216],
                vec![1, 29, 48, 167787520, 1493172224, 16777216]
            ]
        );
    }

    #[test]
    fn test_parse_yaml_from_string() {
        let yaml_content = r#"
        frontends:
          - name: "frontend0"
            id: 0
            guests:
              - name: "guest0"
                id: 0
                ram_addr: 0x60000000
                ram_size: 0x01000000
                socket_path: "/root/"
                devices:
                  - name: "device0"
                    id: 0
                    type: "rng"
                    irq: 0x2f
                    addr: 0xa003e00
              - name: "guest1"
                id: 1
                ram_addr: 0x61000000
                ram_size: 0x01000000
                socket_path: "/root/"
                devices:
                  - name: "device1"
                    id: 1
                    type: "i2c"
                    irq: 0x2e
                    addr: 0xa003c00
                
    "#;
        let frontends: ConfigFrontends = serde_yaml::from_str(&yaml_content).unwrap();

        let expected_frontends = ConfigFrontends {
            frontends: vec![ConfigFrontend {
                name: "frontend0".to_string(),
                id: 0,
                guests: vec![
                    ConfigGuest {
                        name: "guest0".to_string(),
                        id: 0,
                        ram_addr: 0x60000000,
                        ram_size: 0x01000000,
                        socket_path: "/root/".to_string(),
                        devices: vec![ConfigDevice {
                            name: "device0".to_string(),
                            id: 0,
                            device_type: "rng".to_string(),
                            irq: 0x2f,
                            addr: 0xa003e00,
                        }],
                    },
                    ConfigGuest {
                        name: "guest1".to_string(),
                        id: 1,
                        ram_addr: 0x61000000,
                        ram_size: 0x01000000,
                        socket_path: "/root/".to_string(),
                        devices: vec![ConfigDevice {
                            name: "device1".to_string(),
                            id: 1,
                            device_type: "i2c".to_string(),
                            irq: 0x2e,
                            addr: 0xa003c00,
                        }],
                    },
                ],
            }],
        };

        assert_eq!(frontends, expected_frontends);
    }
}
