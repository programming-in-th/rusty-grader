use lapin::{Channel, ConnectionProperties};

use crate::cfg::RabbitMqConfig;

pub async fn get_channel(
    rmq_config: &RabbitMqConfig,
) -> Result<Channel, Box<dyn std::error::Error>> {
    let addr = if rmq_config.vhost == "/" {
        format!(
            "amqp://{username}:{password}@{host}:{port}",
            username = rmq_config.username,
            password = rmq_config.password,
            host = rmq_config.host,
            port = rmq_config.port
        )
    } else {
        format!(
            "amqp://{username}:{password}@{host}:{port}/{vhost}",
            username = rmq_config.username,
            password = rmq_config.password,
            host = rmq_config.host,
            port = rmq_config.port,
            vhost = rmq_config.vhost
        )
    };

    let conn = lapin::Connection::connect(&addr, ConnectionProperties::default()).await?;

    let channel = conn.create_channel().await?;

    Ok(channel)
}
