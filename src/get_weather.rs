use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "http://t.weather.sojson.com/api/weather/city/101270101";
    let body = reqwest::get(url).await?.text().await?;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    let temperature = &json["data"]["wendu"];
    let weather = &json["data"]["forecast"][0]["type"];
    println!("成都今天的天气是：{}，温度是：{}", weather, temperature);
    Ok(())
}