import login_info as info

from selenium import webdriver
from webdriver_manager.chrome import ChromeDriverManager
from selenium.webdriver.common.by import By

import sys
args = sys.argv

driver = webdriver.Chrome(ChromeDriverManager().install())
driver.implicitly_wait(3)

def login_to_time_record(args):

    url = args[1]
    login = args[2]
    if url == 'ncta':
        driver.get(info.URL_LOGIN['ncta'])
        input_id = driver.find_element(by=By.NAME,value='uid')
        input_password = driver.find_element(by=By.NAME,value='pwd')

    elif url == 'indigo':
        driver.get(info.URL_LOGIN['indigo'])
        input_id = driver.find_element(by=By.NAME,value='LoginID')
        input_password = driver.find_element(by=By.NAME,value='Password')

    elif url == 'returning':
        driver.get(info.URL_LOGIN['returning'])
        input_id = driver.find_element(by=By.XPATH,value="//input[@id='loginId']")
        input_password = driver.find_element(by=By.XPATH,value="//input[@id='password']")

    login_id = ''
    login_pw = ''

    if login == 'nitac':
        login_id = info.nitac['UID']
        login_pw = info.nitac['PW']

    elif login == 'r_nitac':
        login_id = info.r_nitac['UID']
        login_pw = info.r_nitac['PW']



    input_id.send_keys(login_id)
    input_password.send_keys(login_pw)

    input_password.submit()

    print("------------Success-------------\n")

login_to_time_record(args)