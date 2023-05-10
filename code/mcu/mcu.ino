#include <SPI.h>
#include <WiFiNINA.h>
//Set low for left
const int PAN_1 = 19;
//Set low for right
const int PAN_2 = 14;
//Set low for up
const int TILT_1 = 3;
//Set low for down
const int TILT_2 = 10;

const int8_t TILT_UP = 0;
const int8_t TILT_DOWN = 1;
const int8_t TILT_OFF = 2;
const int8_t PAN_RIGHT = 3;
const int8_t PAN_LEFT = 4;
const int8_t PAN_OFF = 5;

/*
  WiFi Web Server

 A simple web server that shows the value of the analog input pins.

 This example is written for a network using WPA encryption. For
 WEP or WPA, change the WiFi.begin() call accordingly.

 Circuit:
 * Analog inputs attached to pins A0 through A5 (optional)

 created 13 July 2010
 by dlf (Metodo2 srl)
 modified 31 May 2012
 by Tom Igoe

 */

#include <SPI.h>
#include <WiFiNINA.h>


#include "arduino_secrets.h" 
///////please enter your sensitive data in the Secret tab/arduino_secrets.h
char ssid[] = SECRET_SSID;        // your network SSID (name)
char pass[] = SECRET_PASS;    // your network password (use for WPA, or use as key for WEP)
int keyIndex = 0;                 // your network key index number (needed only for WEP)

int status = WL_IDLE_STATUS;

WiFiServer server(1000);

void setup() {
  pinMode(PAN_1, OUTPUT);
  pinMode(PAN_2, OUTPUT);
  pinMode(TILT_1, OUTPUT);
  pinMode(TILT_2, OUTPUT);
  //Initialize serial and wait for port to open:
  Serial.begin(9600);
  delay(1000);

  // check for the WiFi module:
  if (WiFi.status() == WL_NO_MODULE) {
    Serial.println("Communication with WiFi module failed!");
    // don't continue
    while (true);
  }

  String fv = WiFi.firmwareVersion();
  if (fv < WIFI_FIRMWARE_LATEST_VERSION) {
    Serial.println("Please upgrade the firmware");
  }

  // attempt to connect to WiFi network:
  while (status != WL_CONNECTED) {
    Serial.print("Attempting to connect to SSID: ");
    Serial.println(ssid);
    // Connect to WPA/WPA2 network. Change this line if using open or WEP network:
    status = WiFi.begin(ssid, pass);

    // wait 10 seconds for connection:
    delay(10000);
  }
  server.begin();
  // you're connected now, so print out the status:
  printWifiStatus();
}


void loop() {
  // listen for incoming clients
  WiFiClient client = server.available();
  if (client) {
    Serial.println("new client");
    while (client.connected()) {
      if (client.available()) {
        int8_t cmd = client.read();
        Serial.println(cmd);
        if (cmd == TILT_UP){
          digitalWrite(TILT_1, LOW);
        }
        if (cmd == TILT_DOWN){
          digitalWrite(TILT_2, LOW);
        }        
        if (cmd == TILT_OFF){
          digitalWrite(TILT_1, HIGH);
          digitalWrite(TILT_2, HIGH);
        }
        if (cmd == PAN_RIGHT){
          digitalWrite(PAN_2, LOW);
        }
        if (cmd == PAN_LEFT){
          digitalWrite(PAN_1, LOW);
        }        
        if (cmd == PAN_OFF){
          digitalWrite(PAN_1, HIGH);
          digitalWrite(PAN_2, HIGH);
        }
      }
    }
    // give the web browser time to receive the data
    delay(1);

    // close the connection:
    client.stop();
    Serial.println("client disconnected");
  }
}


void printWifiStatus() {
  // print the SSID of the network you're attached to:
  Serial.print("SSID: ");
  Serial.println(WiFi.SSID());

  // print your board's IP address:
  IPAddress ip = WiFi.localIP();
  Serial.print("IP Address: ");
  Serial.println(ip);

  // print the received signal strength:
  long rssi = WiFi.RSSI();
  Serial.print("signal strength (RSSI):");
  Serial.print(rssi);
  Serial.println(" dBm");
}
