use crate::util::qr::QRError;
use crate::util::Chomp;

pub fn data(input: Vec<u8>, version: u32) -> Result<Vec<u8>, QRError> {
    let mut chomp = Chomp::new(input);
    let mut result = Vec::<u8>::new();

    while let Some(mode) = chomp.chomp(4) {
        match mode {
            0b0001 => result.extend(numeric(&mut chomp, version)?),
            0b0010 => result.extend(alphanumeric(&mut chomp, version)?),
            0b0100 => result.extend(eight_bit(&mut chomp, version)?),
            0b0000 => break,
            _ => {
                return Err(QRError {
                    msg: format!("Mode {:04b} not yet implemented.", mode),
                })
            }
        }
    }

    Ok(result)
}

fn numeric(chomp: &mut Chomp, version: u32) -> Result<Vec<u8>, QRError> {
    let length_bits = match version {
        1..=9 => 10,
        10..=26 => 12,
        27..=40 => 14,
        _ => {
            return Err(QRError {
                msg: format!("Unknown version {}", version),
            });
        }
    };

    let mut length = chomp.chomp_or_u16(
        length_bits,
        QRError {
            msg: format!("Could not read {} bits for numeric length", length_bits),
        },
    )?;

    let mut result = Vec::<u8>::new();

    while length > 0 {
        if length >= 3 {
            let digits = read_bits_u16(chomp, 10)?;
            result.extend(&digits.to_be_bytes());

            length -= 3;
            continue;
        }

        if length == 2 {
            let digits = read_bits_u16(chomp, 7)?;
            result.extend(&digits.to_be_bytes());

            break;
        }

        if length == 1 {
            let digits = read_bits_u16(chomp, 4)?;
            result.extend(&digits.to_be_bytes());

            break;
        }
    }

    debug!("NUMERIC {:?}", result);

    Ok(result)
}

const ALPHANUMERIC: [char; 45] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ' ', '$',
    '%', '*', '+', '-', '.', '/', ':',
];

fn alphanumeric(chomp: &mut Chomp, version: u32) -> Result<Vec<u8>, QRError> {
    let length_bits = match version {
        1..=9 => 9,
        10..=26 => 11,
        27..=40 => 13,
        _ => {
            return Err(QRError {
                msg: format!("Unknown version {}", version),
            });
        }
    };

    let mut length = chomp.chomp_or_u16(
        length_bits,
        QRError {
            msg: format!(
                "Could not read {} bits for alphanumeric length",
                length_bits
            ),
        },
    )?;

    let mut result = Vec::<u8>::new();

    while length > 0 {
        if length >= 2 {
            let chars = read_bits_u16(chomp, 11)?;
            result.push(ALPHANUMERIC[chars as usize / 45] as u8);
            result.push(ALPHANUMERIC[chars as usize % 45] as u8);

            length -= 2;
            continue;
        }

        if length == 1 {
            let chars = read_bits_u16(chomp, 6)?;
            result.push(ALPHANUMERIC[chars as usize] as u8);

            break;
        }
    }

    debug!("ALPHANUMERIC {:?}", result);

    Ok(result)
}

fn eight_bit(chomp: &mut Chomp, version: u32) -> Result<Vec<u8>, QRError> {
    let length_bits = match version {
        1..=9 => 8,
        10..=26 => 16,
        27..=40 => 16,
        _ => {
            return Err(QRError {
                msg: format!("Unknown version {}", version),
            });
        }
    };

    let length = chomp.chomp_or_u16(
        length_bits,
        QRError {
            msg: format!(
                "Could not read {} bits for alphanumeric length",
                length_bits
            ),
        },
    )?;

    let mut result = Vec::<u8>::new();

    for _ in 0..length {
        result.push(read_bits(chomp, 8)?);
    }

    debug!("EIGHT BIT {:?}", result);

    Ok(result)
}

fn read_bits(chomp: &mut Chomp, bits: u8) -> Result<u8, QRError> {
    chomp.chomp_or(
        bits,
        QRError {
            msg: format!("Could not read {} bits", bits),
        },
    )
}

fn read_bits_u16(chomp: &mut Chomp, bits: u8) -> Result<u16, QRError> {
    chomp.chomp_or_u16(
        bits,
        QRError {
            msg: format!("Could not read {} bits", bits),
        },
    )
}
