from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.chrome.service import Service as ChromeService
from webdriver_manager.chrome import ChromeDriverManager
import time

# Configuración del navegador
driver = webdriver.Chrome(service=ChromeService(ChromeDriverManager().install()))
driver.get("http://localhost:3000")

# Prueba de reserva de asientos
driver.find_element(By.ID, "cantidad").send_keys("3")
driver.find_element(By.ID, "precio_max").send_keys("100")
driver.find_element(By.TAG_NAME, "button").click()
time.sleep(2)

# Cambiar a la página de estado
driver.get("http://localhost:3000/estado")
time.sleep(2)

# Validar el estado de algunos asientos
asientos = driver.find_elements(By.CLASS_NAME, "asiento")
for asiento in asientos:
    print(asiento.text, asiento.get_attribute("class"))

# Cerrar el navegador
driver.quit()
