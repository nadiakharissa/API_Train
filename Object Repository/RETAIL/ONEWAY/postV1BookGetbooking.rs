<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>postV1BookGetbooking</name>
   <tag></tag>
   <elementGuidId>8315ee9b-4717-4993-b98f-5c59472a270d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;searchKey\&quot;: \&quot;string\&quot;,\n  \&quot;customerType\&quot;: \&quot;RETAIL\&quot;,\n  \&quot;selectedJourneys\&quot;: [\n    {\n      \&quot;journeyType\&quot;: \&quot;ONEWAY\&quot;,\n      \&quot;origin\&quot;: \&quot;gmr\&quot;,\n      \&quot;destination\&quot;: \&quot;bd\&quot;,\n      \&quot;departureDate\&quot;: {\n        \&quot;day\&quot;: \&quot;20\&quot;,\n        \&quot;month\&quot;: \&quot;10\&quot;,\n        \&quot;year\&quot;: \&quot;2019\&quot;\n      },\n      \&quot;subclass\&quot;: \&quot;A\&quot;,\n      \&quot;trainNo\&quot;: \&quot;10987\&quot;\n    }\n  ],\n  \&quot;numOfAdult\&quot;: 1,\n  \&quot;numOfInfant\&quot;: 1,\n  \&quot;contact\&quot;: {\n    \&quot;fullName\&quot;: \&quot;Ari Lutifiani\&quot;,\n    \&quot;email\&quot;: \&quot;juliaamalina@gmail.com\&quot;,\n    \&quot;mobilePhone\&quot;: \&quot;08256172381271\&quot;,\n    \&quot;title\&quot;: \&quot;Mrs\&quot;\n  },\n  \&quot;passenger\&quot;: {\n    \&quot;adult\&quot;: [\n      {\n        \&quot;firstName\&quot;: \&quot;Ari\&quot;,\n        \&quot;lastName\&quot;: \&quot;Lutfiani\&quot;,\n        \&quot;phoneNumber\&quot;: \&quot;0826371929179\&quot;,\n        \&quot;identityNumber\&quot;: \&quot;021812737813\&quot;,\n        \&quot;dateOfBirth\&quot;: {\n          \&quot;day\&quot;: \&quot;07\&quot;,\n          \&quot;month\&quot;: \&quot;09\&quot;,\n          \&quot;year\&quot;: \&quot;1993\&quot;\n        }\n      }\n    ],\n    \&quot;infant\&quot;: [\n      {\n        \&quot;firstName\&quot;: \&quot;Gea\&quot;,\n        \&quot;lastName\&quot;: \&quot;Kusuma\&quot;,\n        \&quot;dateOfBirth\&quot;: {\n          \&quot;day\&quot;: \&quot;23\&quot;,\n          \&quot;month\&quot;: \&quot;03\&quot;,\n          \&quot;year\&quot;: \&quot;2018\&quot;\n        }\n      }\n    ]\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
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
   <restUrl>https://test-katrina.icore.studio/train/api/v1/book/getBooking</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyElementPropertyValue(response, 'itineraries.journeys[0].bookCode', &quot;YFU2XKG&quot;)


WS.verifyElementPropertyValue(response, 'itineraries.journeys[0].pnrID', 3044)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
