//	bearerToken     = settings.value("Network/token", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoiNWY4NjE0YmNhZjhjODU2NDE0MzdmZTE3IiwiaWF0IjoxNjAyNzE4MjAyLCJhdWQiOiJiaXRmbHV4LmFpIiwiaXNzIjoiYml0Zmx1eC5haSJ9.K-ssHkaX02h0HkDBANUIFBvbnL1cwImtufr4VEzxS-c").toString();
//void Transmitter::sendSingleBodyPartPacket(QByteArray message, QString endpoint, bool syncWait)
//{
	// This is the simpler approach of sending data, which will only supply form/body data.
	// super useful for messages, events, and completely json based data types that don't require compression.

//	QNetworkRequest req(QUrl(QString("%1/%2").arg(destServer, endpoint)));
//	req.setHeader(QNetworkRequest::ContentTypeHeader, QVariant("application/json"));


//	req.setRawHeader(QByteArray("Authorization"), QByteArray(QString("Bearer %1").arg(bearerToken).toUtf8()));
//	QNetworkReply* replypointer = netmanager->post(req, message);
//	outstandingPackets++;

	// We do this because the start and stop messages need to be sync, and the file uploads don't. this guarantees
	// that the start messages finish before the barrage of file parts start going.
//	if (syncWait)
//	{
//		SyncWaitForPacket(replypointer);
//	}
//}
//void Transmitter::sendStatData(QByteArray messageContents)
//  {
//  	QJsonDocument mdata;
//  	QJsonObject metaobject;
//  	metaobject["timeStamp"] = QDateTime::currentDateTimeUtc().toString();
//  	metaobject["deviceUid"] = deviceId;
//  	metaobject["machineId"] = machineIdCached;
//  	metaobject["format"] = "json";
//  	metaobject["type"] = "summary";
//  	metaobject["version"] = QString::number(MESSAGE_VERSION);
//  	metaobject["stats"] = QString(messageContents);
//  	metaobject["class"] = "statistics";
//  	mdata.setObject(metaobject);
//  	sendStatMessage(mdata.toJson(QJsonDocument::Compact));
//  }



//
use restson;
#[macro_use]
use serde_json::Value;
use serde_derive::Serialize;
use serde_derive::Deserialize;
//use std::error::Error;
//use futures::executor::block_on;

//#[derive(Default, Debug, Serialize, Deserialize)]

//use restson::blocking::RestClient;
use restson::{RestClient,RestPath,Error};
use reqwest;

// Data structure that matches with REST API JSON
#[derive(Serialize,Deserialize,Debug)]
struct HttpBinAnything {
    method: String,
    url: String,
}

// Path of the REST endpoint: e.g. http://<baseurl>/anything
impl RestPath<()> for HttpBinAnything {
    fn get_path(_: ()) -> Result<String,Error> { Ok(String::from("anything")) }
}


pub fn transtest1() {
    // Create new client with API base URL
    let mut client = RestClient::new_blocking("http://httpbin.org").unwrap();

    // GET http://httpbin.org/anything and deserialize the result automatically
    let data: HttpBinAnything = client.get(()).unwrap();
    println!("{:?}", data);
    println!("addd");
}

pub fn transtest() {
    let bearerToken = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoiNWY4NjE0YmNhZjhjODU2NDE0MzdmZTE3IiwiaWF0IjoxNjAyNzE4MjAyLCJhdWQiOiJiaXRmbHV4LmFpIiwiaXNzIjoiYml0Zmx1eC5haSJ9.K-ssHkaX02h0HkDBANUIFBvbnL1cwImtufr4VEzxS-c";

    let client = reqwest::blocking::Client::new();
    let cp = client.post("https://flux.bitflux.ai");
    let cpj = cp.json(&serde_json::json!({
                      "title": "Reqwest.rs",
                      "body": "https://docs.rs/reqwest",
                      "userId": 1
                      }));
    let cpjs = cpj.send();
    //let cpjs: serde_json::Value = cpj.send();
    //let cpjsj = cpjs.json();
    //let echo_json: serde_json::Value = reqwest::blocking::Client::new()
    println!("{:#?}", cpjs);
}