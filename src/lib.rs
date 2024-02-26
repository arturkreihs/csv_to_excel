use csv::Reader;
use rust_xlsxwriter::Workbook;
pub use rust_xlsxwriter::XlsxError;

pub fn convert(csv: &str) -> Result<Vec<u8>, XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let mut rdr = Reader::from_reader(csv.as_bytes());
    for (row_index, row) in rdr.records().enumerate() {
        if let Ok(record) = row {
            for (col_index, cell) in record.iter().enumerate() {
                worksheet.write_string(row_index as u32, col_index as u16, cell)?;
            }
        }
    }
    workbook.save_to_buffer()
}
