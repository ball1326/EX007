import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.warframe.com/landing')

WebUI.click(findTestObject('Object Repository/Page_Warframe Ninjas Play Free/button_Accept All Cookies_onetrust-close-bt_a4e8e4'))

WebUI.click(findTestObject('Object Repository/Page_Warframe Ninjas Play Free/a_PLAY FREE NOW'))

WebUI.click(findTestObject('Object Repository/Page_Warframe Ninjas Play Free/a_PC'))

WebUI.setText(findTestObject('Object Repository/Page_Warframe Ninjas Play Free/input_Email Address_email'), 'aticha13264@gmail.com')

WebUI.setText(findTestObject('Object Repository/Page_Warframe Ninjas Play Free/input_Confirm Email_email_confirm'), 'aticha13264@gmail.com')

WebUI.setText(findTestObject('Object Repository/Page_Warframe Ninjas Play Free/input_Alias_alias'), 'maika888')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Warframe Ninjas Play Free/input_Password_password'), 'jHtYswa4RHsXlaLMpRlspg==')

WebUI.click(findTestObject('Object Repository/Page_Warframe Ninjas Play Free/form_Email Address                         _0ae75f'))

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Warframe Ninjas Play Free/input_Password_password'), 'wMwijjdFB3My1SXCmCpu6g==')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Warframe Ninjas Play Free/input_Confirm Password_password_confirm'), 
    'blj4I1b1N0BpSOCt2ZAQeQ==')

WebUI.click(findTestObject('Object Repository/Page_Warframe Ninjas Play Free/form_Email Address                         _0ae75f_1'))

WebUI.click(findTestObject('Object Repository/Page_Warframe Ninjas Play Free/input_concat(Your confirmed password doesn,_7d1f74'))

WebUI.click(findTestObject('Object Repository/Page_Warframe Ninjas Play Free/input_concat(Your confirmed password doesn,_03be92'))

WebUI.click(findTestObject('Object Repository/Page_Warframe Ninjas Play Free/div_Join Now'))

