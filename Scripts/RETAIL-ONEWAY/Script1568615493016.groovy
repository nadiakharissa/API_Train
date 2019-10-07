import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

def booking = WS.sendRequest(findTestObject('RETAIL/ONEWAY/postV1BookGetbooking'))

WS.sendRequest(findTestObject('RETAIL/ONEWAY/postV1SeatChangeseat'))

def slurper = new groovy.json.JsonSlurper()

def result = slurper.parseText(booking.getResponseBodyContent())

def bookingcode = result.itineraries.journeys[0].bookCode

def id = result.itineraries.journeys[0].pnrID

GlobalVariable.bookingcode = bookingcode

GlobalVariable.PNRID = id

println ('booking code is' +GlobalVariable.bookingcode)

println ('PNRID is' +id)

def seatlist = WS.sendRequest(findTestObject('RETAIL/ONEWAY/postV1SeatChangeseat', [('bookingcode') : +bookingcode, ('PNRID') : +id]))

String[][] arraywagonno = seatlist.seat_map
String[][][] arraywagon = seatlist.seat_map
String[][][][] arrayseat = seatlist.seat_map

for (i=0; i<arraywagonno.length; i++){
	println (seatlist.seat_map[0][i])
	
}






