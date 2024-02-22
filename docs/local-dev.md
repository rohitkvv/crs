## How to run mongo-express locally
Mongo express fails to run when added as a service as part of docker compose. So it needs to be run separately using the following command

>> docker run -it --rm \
>>    --name mongo-express \
>>    --network YOUR_NETWORK_BRIDGE_NAME \
>>    -p 8081:8081 \
>>    -e ME_CONFIG_OPTIONS_EDITORTHEME="ambiance" \
>>    -e ME_CONFIG_BASICAUTH_USERNAME="" \
>>    -e ME_CONFIG_MONGODB_URL="mongodb://DB_NAME:27017" \
>>    mongo-express

## How to create a custom bridge network in docker
>> docker network create **YOUR_NETWORK_BRIDGE_NAME**