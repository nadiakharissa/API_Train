<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>postV1SeatChangeseat</name>
   <tag></tag>
   <elementGuidId>10743673-3791-4871-8ede-f54dccda6a96</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;bookCode\&quot;: \&quot;B7R25Y0\&quot;,\n  \&quot;pnrID\&quot;: \&quot;3060\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic YjgwNzcyOTMtODg2YS1iOTEwLWE3ZjktMTM2MGNmMWNhMjRhOjEyOWZhOGM2LWM0MjYtMWNjNi1iMjllLWRhYWQxYTdlMjVjNA==</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://test-katrina.icore.studio/train/api/v1/seat/changeSeat</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.bookingcode</defaultValue>
      <description></description>
      <id>cd02f0b1-4bc5-4dc1-9663-3325248c29f5</id>
      <masked>false</masked>
      <name>bookingcode</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PNRID</defaultValue>
      <description></description>
      <id>2af69b01-feb2-4c5b-ad0d-cfc0b857ea88</id>
      <masked>false</masked>
      <name>PNRID</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyElementPropertyValue(response, 'seat_map[0][0]', '')
WS.verifyElementPropertyValue(response, 'seat_map', '')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
