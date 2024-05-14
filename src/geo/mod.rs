use reverse_geocoder::ReverseGeocoder;

pub fn convert_gps(data:(f64,f64,f64,String))-> f64{
	let s:f64 = data.2 / 60.0;
	let m_t:f64  = data.1 + s;
	let m: f64  = m_t/60.0;
	let mut decimal:f64 = data.0 + m;
	if &data.3 == "N" || &data.3 == "E"{
		return decimal;
	}
	else {
		decimal = -decimal;
	}
	decimal
}
pub fn get_location(latitude:f64,longitude:f64) -> String {
	let geocoder = ReverseGeocoder::new();
	let coords = (latitude, longitude);
	let search_result = geocoder.search(coords);
	println!("{}",search_result.record.name);
	let locality= &search_result.record.name;
	return locality.to_string()
}
