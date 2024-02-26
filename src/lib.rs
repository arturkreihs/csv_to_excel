use csv::Reader;
use rust_xlsxwriter::{ Workbook, Format, Color };
pub use rust_xlsxwriter::XlsxError;

pub fn convert(csv: &str) -> Result<Vec<u8>, XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let mut rdr = Reader::from_reader(csv.as_bytes());
    let headers_format = Format::new()
        .set_bold()
        .set_background_color(Color::Silver);
    let headers = rdr.headers()
        .map_err(|_| XlsxError::CustomError("headers can't be obtained".into()))?;
    for (col_index, cell) in headers.iter().enumerate() {
        worksheet.write_string_with_format(0, col_index as u16, cell, &headers_format)?;
    }
    for (row_index, row) in rdr.records().enumerate() {
        if let Ok(record) = row {
            for (col_index, cell) in record.iter().enumerate() {
                worksheet.write_string((row_index + 1) as u32, col_index as u16, cell)?;
            }
        }
    }
    workbook.save_to_buffer()
}
