-- Create separate databases for each microservice
CREATE DATABASE bbps_auth;
CREATE DATABASE bbps_curriculum;
CREATE DATABASE bbps_assessment;
CREATE DATABASE bbps_content;
CREATE DATABASE bbps_live_class;
CREATE DATABASE bbps_notification;
CREATE DATABASE bbps_payment;

-- Grant permissions
GRANT ALL PRIVILEGES ON DATABASE bbps_auth TO bbps;
GRANT ALL PRIVILEGES ON DATABASE bbps_curriculum TO bbps;
GRANT ALL PRIVILEGES ON DATABASE bbps_assessment TO bbps;
GRANT ALL PRIVILEGES ON DATABASE bbps_content TO bbps;
GRANT ALL PRIVILEGES ON DATABASE bbps_live_class TO bbps;
GRANT ALL PRIVILEGES ON DATABASE bbps_notification TO bbps;
GRANT ALL PRIVILEGES ON DATABASE bbps_payment TO bbps;
