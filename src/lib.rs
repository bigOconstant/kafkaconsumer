
//extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate rustoculuslogger;
use std::env;

// #[macro_use]
// extern crate rocket;

extern crate serde;


#[macro_use]
extern crate lazy_static;
extern crate base64;
extern crate reqwest;
// extern crate chrono;

//use clap::{App, Arg};
use futures::stream::Stream;

use rdkafka::message::{Message, Headers};
use rdkafka::client::ClientContext;
use rdkafka::consumer::{Consumer, ConsumerContext, CommitMode, Rebalance};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
//use rdkafka::util::get_rdkafka_version;
use rdkafka::error::KafkaResult;

use std::error::Error;

//mod example_utils;
//use crate::example_utils::setup_logger;

// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
struct CustomContext;



impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        rustoculuslogger::LogObject::new()
                            .msg("Pre rebalance".to_string())
                            .op("ConsumerContext".to_string())
                            .lvl("Info".to_string())
                            //.optionaldata(rebalance.to_string())
                            .print();
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        rustoculuslogger::LogObject::new()
                            .msg("Post rebalance".to_string())
                            .op("ConsumerContext".to_string())
                            .lvl("Info".to_string())
                            //.optionaldata(rebalance.to_string())
                            .print();
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: *mut rdkafka_sys::RDKafkaTopicPartitionList) {
        rustoculuslogger::LogObject::new()
                            .msg("Committing offsets".to_string())
                            .op("ConsumerContext".to_string())
                            .lvl("Info".to_string())
                            //.optionaldata(result.to_string())
                            .print();
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;
//data::SC.sasluser = sasluser.clone();
                                //  data::SC.saslpassword = saslpassword.clone();
//Pasing in handler function to handle what to do with the item.
pub fn consume_kafka_messages(brokers: String, group_id: String, topics: String,sasluser:String,saslpassword:String,f:fn(input:&str)->String) {
    let context = CustomContext;
    
    // println!("brokers: {:?}",brokers);
    // println!("group_id:{:?}",group_id);
    // println!("topics {:?}",topics);


    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", &group_id)
        .set("bootstrap.servers", &brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("security.protocol","SASL_PLAINTEXT")
        .set("sasl.mechanism","PLAIN")
        .set("sasl.mechanisms","PLAIN")
        .set("sasl.username",&sasluser)
        .set("sasl.password",&saslpassword)
        //.set("sasl.kerberos.keytab","doesntExist.txt")
        //.set("statistics.interval.ms", "30000")
        //.set("auto.offset.reset", "smallest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    consumer.subscribe(&[topics.as_str()][..])
        .expect("Can't subscribe to specified topics");

    // consumer.start() returns a stream. The stream can be used ot chain together expensive steps,
    // such as complex computations on a thread pool or asynchronous IO.
    let message_stream = consumer.start();
    for message in message_stream.wait() {
  
        match message {
            Err(_) => {
                rustoculuslogger::LogObject::new()
                            .msg("Error while reading from stream.".to_string())
                            .op("consume_kafka_messages".to_string())
                            .lvl("error".to_string())
                            .print();
            },
            Ok(Err(e)) => {
                rustoculuslogger::LogObject::new()
                            .msg("Kafka error".to_string())
                            .op("consume_kafka_messages".to_string())
                            .lvl("error".to_string())
                            .optionaldata(e.to_string())
                            .print();
            },
            Ok(Ok(m)) => {
                   match m.payload_view::<str>() {
                    None => "".to_string(),
                    Some(Ok(s)) => {
                        f(s)
                        // let sss = get_injestor(s);
                        // send_request_out(sss,f);
                        // s
                    },
                    Some(Err(e)) => {
                        rustoculuslogger::LogObject::new()
                            .msg("Error reading message".to_string())
                            .op("consume_kafka_messages".to_string())
                            .lvl("error".to_string())
                            .optionaldata(e.to_string())
                            .print();
                        "".to_string()
                    },
                };

                if let Some(headers) = m.headers() {
                    for i in 0..headers.count() {
                        let header = headers.get(i).unwrap();
                       // info!("  Header {:#?}: {:?}", header.0, header.1);
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            },
        };
    }
}






#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        rustoculuslogger::LogObject::new()
                            .msg("Committing offsets".to_string())
                            .op("ConsumerContext".to_string())
                            .lvl("Info".to_string())
                            //.optionaldata(result.to_string())
                            .print();
        assert_eq!(2 + 2, 4);
    }
}
