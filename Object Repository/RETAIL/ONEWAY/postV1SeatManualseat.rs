<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>postV1SeatManualseat</name>
   <tag></tag>
   <elementGuidId>10cc7be3-581f-4668-ab25-a19757cf81cd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;pnrId\&quot;: \&quot;string\&quot;,\n  \&quot;bookCode\&quot;: \&quot;${bookingcode}\&quot;,\n  \&quot;wagonCode\&quot;: \&quot;${wagoncode}\&quot;,\n  \&quot;wagonNo\&quot;: \&quot;${wagonno}\&quot;,\n  \&quot;seat\&quot;: \&quot;${seat}\&quot;\n}&quot;,
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
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://test-katrina.icore.studio/train/api/v1/seat/manualSeat</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.bookingcode</defaultValue>
      <description></description>
      <id>6f70131f-5f1e-403b-be13-80ac4864497d</id>
      <masked>false</masked>
      <name>bookingcode</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PNRID</defaultValue>
      <description></description>
      <id>0ddbd83b-7a91-44ba-a69a-5aebcc4c94ee</id>
      <masked>false</masked>
      <name>PNRID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.wagonno</defaultValue>
      <description></description>
      <id>2ad0c638-846e-429c-8307-7508e58f72d7</id>
      <masked>false</masked>
      <name>Wagon</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.wagoncode</defaultValue>
      <description></description>
      <id>d5970f59-aad4-47e0-91c1-a8be9d27960b</id>
      <masked>false</masked>
      <name>wagoncode</name>
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
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
