"""
This script is auto send system for authorized form about
health check.
This script need login_info.py , chromedrivermanager , 
and selenium.
Please write address and password in login_info.py
"""

from selenium import webdriver
from webdriver_manager.chrome import ChromeDriverManager
import login_info as info
from selenium.webdriver.common.by import By


driver = webdriver.Chrome(ChromeDriverManager().install())
driver.implicitly_wait(3)

def send_health():

    send_url = "https://docs.google.com/forms/d/e/1FAIpQLSc6-1J5B5UkTbTwj9werYh5DSr4J1K9kHv6HiGeVQ-qf9tdDg/formResponse?usp=pp_url&entry.675230857=平熱&entry.745928972=1%EF%BC%8E%E7%95%B0%E5%B8%B8%E3%81%AA%E3%81%97"

    driver.get(send_url)
    """
    input type = password
    name = password
    input type = email
    name = identifier
    id = identifierId
    """

    login_id = info.google['ars']
    login_pass = info.google['pwd']


    input_id = driver.find_element(by=By.XPATH,value="//input[@id='identifierId']")
    input_id.send_keys(login_id)
    f_button = driver.find_element(by=By.XPATH,value="//*[@id='identifierNext']")
    f_button.click()
    driver.implicitly_wait(5)
    input_password = driver.find_element(by=By.XPATH,value="//input[@name='password']")
    input_password.send_keys(login_pass)
    s_button = driver.find_element(by=By.XPATH,value="//*[@id='passwordNext']")
    s_button.click()
    driver.implicitly_wait(5)

    f_next = driver.find_element(by=By.XPATH,value="//*[@class='uArJ5e UQuaGc YhQJj zo8FOc ctEux']")
    f_next.click()
    driver.implicitly_wait(5)
    s_next = driver.find_element(by=By.XPATH,value="//div[2]/div/div[3]/div/div[1]/div[2][@class='uArJ5e UQuaGc YhQJj zo8FOc ctEux']")
    s_next.click()
    driver.implicitly_wait(5)
    form_send = driver.find_element(by=By.XPATH,value="//div[2]/div/div[3]/div/div[1]/div[2][@class='uArJ5e UQuaGc Y5sE8d VkkpIf QvWxOd']")
    form_send.click()
    driver.implicitly_wait(3)

send_health()