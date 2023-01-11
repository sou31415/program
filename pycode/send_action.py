"""
This script send my action of before day.
"""

from selenium import webdriver
from webdriver_manager.chrome import ChromeDriverManager
import login_info as info

send_url = "https://docs.google.com/forms/d/e/1FAIpQLSf9riX1UuXxnt21djbs5zTrEM8K952v2iVBij4iC-piYplOKg/formResponse?usp=pp_url&entry.1364072590=変わりはない！&entry.868343064=いいえ"
driver = webdriver.Chrome(ChromeDriverManager().install())
driver.implicitly_wait(3)

def action_send():

    login_id = info.google['ars']
    login_pass = info.google['pwd']
    driver.get(send_url)

    input_id = driver.find_element_by_xpath("//input[@id='identifierId']")
    input_id.send_keys(login_id)
    f_button = driver.find_element_by_xpath("//*[@id='identifierNext']")
    f_button.click()
    driver.implicitly_wait(5)
    input_password = driver.find_element_by_xpath("//input[@name='password']")
    input_password.send_keys(login_pass)
    s_button = driver.find_element_by_xpath("//*[@id='passwordNext']")
    s_button.click()
    driver.implicitly_wait(5)

    f_next = driver.find_element_by_xpath("//*[@class='uArJ5e UQuaGc YhQJj zo8FOc ctEux']")
    f_next.click()
    driver.implicitly_wait(3)
    switch = driver.find_element_by_xpath("/html/body/div/div[2]/form/div[2]/div/div[3]/div[1]/label/div/div[1][@class='LsSwGf SWVgue FXLARc Y4klN']")
    switch.click()
    form_send = driver.find_element_by_xpath("//div[2]/div/div[3]/div[2]/div[1]/div[2]/span/span[@class='NPEfkd RveJvd snByac']")
    form_send.click()
    driver.implicitly_wait(10)

action_send()