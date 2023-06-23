use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};





#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    password: String,
}

impl User {
    // Function to check if the provided username and password are valid
    fn is_valid(&self, username: &str, password: &str) -> bool {
        self.username == username && self.password == password
    }
}


 
#[get("/")]
async fn login() -> impl Responder {
    let html = r#"
        <html>
            <head>
                <title>Login</title>
                <style>
                    body {
                        background-color: #f2f2f2;
                        font-family: Arial, sans-serif;
                        margin: 0;
                        padding: 0;
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        height: 100vh;
                    }
                    
                    .container {
                        text-align: center;
                    }
                    
                    h1 {
                        color: #333333;
                    }
                    
                    form {
                        background-color: #ffffff;
                        padding: 20px;
                        border-radius: 5px;
                        box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
                    }
                    
                    input[type="text"],
                    input[type="password"] {
                        width: 100%;
                        padding: 10px;
                        margin-bottom: 10px;
                        border: 1px solid #cccccc;
                        border-radius: 4px;
                    }
                    
                    button[type="submit"] {
                        width: 100%;
                        padding: 10px;
                        background-color: #4caf50;
                        border: none;
                        border-radius: 4px;
                        color: #ffffff;
                        font-weight: bold;
                        cursor: pointer;
                    }
                </style>
            </head>
            <body>
                <div class="container">
                    <h1>Login</h1>
                    <form method="post" action="/login">
                        <input type="text" name="username" placeholder="Username" required><br>
                        <input type="password" name="password" placeholder="Password" required><br>
                        <button type="submit">Login</button>
                    </form>
                </div>
            </body>
        </html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

 
#[post("/login")]
async fn process_login(data: web::Form<User>) -> impl Responder {
if data.is_valid("admin", "password") {
        // Redirect to the main menu if the login is successful
        HttpResponse::Found()
            .header("Location", "/main_menu")
            .finish()
    } else {
        // Display an error message if the login is unsuccessful
        let html = r#"
            <html>
                <head>
                    <title>Login Failed</title>
                </head>
                <body>
                    <h1>Login Failed</h1>
                    <p>Invalid username or password. Please try again.</p>
                    <a href="/">Go back to login</a>
                </body>
            </html>
        "#;

        HttpResponse::Unauthorized()
            .content_type("text/html")
            .body(html)
    }
}



 #[get("/main_menu")]
async fn main_menu() -> impl Responder {
    let html = r#"
        <html>
            <head>
                <title>Menu</title>
                <style>
                    body {
                        display: flex;
                        flex-direction: column;
                        margin: 0;
                        padding: 0;
                    }

                    .top-panel {
                        height: 2cm;
                        width: 100%;
                        background-color: darkblue;
                        display: flex;
                        justify-content: flex-end;
                        align-items: center;
                        padding-right: 10px;
                    }

                    .top-button {
                        padding: 10px;
                        cursor: pointer;
                        text-align: center;
                        background-color: lightgray;
                        border-radius: 5px;
                    }

                    .top-button:hover {
                        background-color: gray;
                        color: white;
                    }

                    .container {
                        display: flex;
                    }

                    .left-section {
                        width: 4cm;
                        height: calc(100vh - 2cm);
                        background-color: lightblue;
                    }

                    .main-section {
                        flex-grow: 1;
                        background-color: aquamarine;
                        padding: 10px;
                    }

                    .menu-button {
                        padding: 10px;
                        margin-bottom: 5px;
                        cursor: pointer;
                        text-align: center;
                        background-color: blue;
                        border-radius: 5px;
                    }

                    .menu-button:hover {
                        background-color: darkblue;
                        color: white;
                    }

                    .selected {
                        font-weight: bold;
                    }
					
                </style>
                <script>
                    let currentOption = 1;
                    
                    function handleKeyPress(event) {
                        if (event.key === "ArrowDown") {
                            event.preventDefault();
                            goToNextOption();
                        } else if (event.key === "ArrowUp") {
                            event.preventDefault();
                            goToPreviousOption();
                        } else if (event.key === "Enter") {
                            event.preventDefault();
                            goToSubMenu();
                        }
                    }
                    
                    function goToNextOption() {
                        if (currentOption < 9) {
                            currentOption++;
                        }
                        updateMenu();
                    }
                    
                    function goToPreviousOption() {
                        if (currentOption > 1) {
                            currentOption--;
                        }
                        updateMenu();
                    }
                    
                    function goToSubMenu() {
                        const optionNames = [
						    "/main_menu",
                            "/Date and time",
                            "/Display",
                            "/submenu3",
                            "/submenu4",
                            "/submenu5",
                            "/submenu6",
                            "/submenu7",
                            "/submenu8",
                            "/submenu9"
                        ];
                        
                        if (currentOption >= 1 && currentOption <= optionNames.length) {
                            window.location.href = optionNames[currentOption - 1];
                        }
                    }
                    
                    function updateMenu() {
                        const menuDiv = document.getElementById("menu");
                        menuDiv.innerHTML = "";

                        const optionNames = [
						    "Main",
                            "Date and Time",
                            "Display",
                            "LED Control",
                            "Sound",
                            "SNMP Settings",
                            "Open VPN Settings",
                            "Language",
                            "Network",
                            "SIP Account"
                        ];
                        
                        for (let i = 0; i < optionNames.length; i++) {
                            const optionDiv = document.createElement("div");
                            optionDiv.textContent = optionNames[i];
                            optionDiv.classList.add("menu-button");

                            if (i === currentOption - 1) {
                                optionDiv.classList.add("selected");
                            }

                            optionDiv.onclick = function () {
                                currentOption = i + 1;
                                goToSubMenu();
                            };

                            menuDiv.appendChild(optionDiv);
                        }
                    }

                    function goToPage(url) {
                        window.location.href = url;
                    }
                    
                    document.addEventListener('keydown', handleKeyPress);
                    updateMenu();
                </script>
            </head>
            <body>
                <div class="top-panel">
                    
                    <button class="top-button" onclick="goToPage('/')">Quit</button>
                </div>
                <div class="container">
                    <div class="left-section">
                        <div id="menu"></div>
                        
                    </div>
                    <div class="main-section">
                        <h2>Main Section</h2>
                        <p>This is the main section content.</p>
                    </div>
                </div>
                <script>
                    updateMenu();
                </script>
            </body>
        </html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
} 




#[get("/Date and time")]
async fn submenu1() -> impl Responder {
    let html = r#"
        <html>
            <head>
			 <meta charset="UTF-8">
                <title>Menu</title>
                <style>
                    body {
                        display: flex;
                        flex-direction: column;
                        margin: 0;
                        padding: 0;
                    }

                    .top-panel {
                        height: 2cm;
                        width: 100%;
                        background-color: darkblue;
                        display: flex;
                        justify-content: flex-end;
                        align-items: center;
                        padding-right: 10px;
                    }

                    .top-button {
                        padding: 10px;
                        cursor: pointer;
                        text-align: center;
                        background-color: lightgray;
                        border-radius: 5px;
                    }

                    .top-button:hover {
                        background-color: gray;
                        color: white;
                    }

                    .container {
                        display: flex;
                    }

                    .left-section {
                        width: 4cm;
                        height: calc(100vh - 2cm);
                        background-color: lightblue;
                    }

                    .main-section {
                        flex-grow: 1;
                        background-color: aquamarine;
                        padding: 10px;
                    }

                    .menu-button {
                        padding: 10px;
                        margin-bottom: 5px;
                        cursor: pointer;
                        text-align: center;
                        background-color: blue;
                        border-radius: 5px;
                    }

                    .menu-button:hover {
                        background-color: darkblue;
                        color: white;
                    }

                    .selected {
                        font-weight: bold;
                    }
                    
                    .time-format-container {
                        display: flex;
                        align-items: center;
                        margin-bottom: 10px;
                    }
                    
                    .time-format-label {
                        margin-right: 10px;
                    }
                    
                    .time-format-option {
                        display: flex;
                        align-items: center;
                        margin-right: 10px;
                        cursor: pointer;
                    }
                    
                    .time-format-option .circle {
                        height: 20px;
                        width: 20px;
                        border: 2px solid black;
                        border-radius: 50%;
                        margin-right: 5px;
                    }
                    
                    .time-format-option.selected .circle {
                        background-color: blue;
					
                    }
					.language-button {
                        padding: 10px;
                        cursor: pointer;
                        text-align: center;
                        background-color: lightgray;
                        border-radius: 5px;
                        margin-left: 10px;
                    }

                    .language-button:hover {
                        background-color: gray;
                        color: white;
					}
                </style>
                <script>
                    let currentOption = 2;
                    let timeDisplayFormat = "12 hours";
                    let dateDisplayFormat = "yyyy-mm-dd";
                    let timeZone = "";
                    let ntpServer = "";
                    
                    function handleKeyPress(event) {
                        if (event.key === "ArrowDown") {
                            event.preventDefault();
                            goToNextOption();
                        } else if (event.key === "ArrowUp") {
                            event.preventDefault();
                            goToPreviousOption();
                        } else if (event.key === "Enter") {
                            event.preventDefault();
                            goToSubMenu();
                        }
                    }
                    
                    function goToNextOption() {
                        if (currentOption < 9) {
                            currentOption++;
                        }
                        updateMenu();
                    }
                    
                    function goToPreviousOption() {
                        if (currentOption > 1) {
                            currentOption--;
                        }
                        updateMenu();
                    }
                    
                    function goToSubMenu() {
                        const optionNames = [
                            "/main_menu",
                            "/Date and time",
                            "/Display",
                            "/submenu3",
                            "/submenu4",
                            "/submenu5",
                            "/submenu6",
                            "/submenu7",
                            "/submenu8",
                            "/submenu9"
                        ];
                        
                        if (currentOption >= 1 && currentOption <= optionNames.length) {
                            window.location.href = optionNames[currentOption - 1];
                        }
                    }
                    
                    function updateMenu() {
                        const menuDiv = document.getElementById("menu");
                        menuDiv.innerHTML = "";

                        const optionNames = [
                            "Main",
                            "Date and Time",
                            "Display",
                            "LED Control",
                            "Sound",
                            "SNMP Settings",
                            "Open VPN Settings",
                            "Language",
                            "Network",
                            "SIP Account"
                        ];
                        
                        for (let i = 0; i < optionNames.length; i++) {
                            const optionDiv = document.createElement("div");
                            optionDiv.textContent = optionNames[i];
                            optionDiv.classList.add("menu-button");

                            if (i === currentOption - 1) {
                                optionDiv.classList.add("selected");
                            }

                            optionDiv.onclick = function () {
                                currentOption = i + 1;
                                goToSubMenu();
                            };

                            menuDiv.appendChild(optionDiv);
                        }
                    }

                    function goToPage(url) {
                        window.location.href = url;
                    }
                    
                    function toggleTimeDisplayFormat(format) {
                        timeDisplayFormat = format;
                        updateDisplayFormats();
                    }
                    
                    function selectDateDisplayFormat(format) {
                        dateDisplayFormat = format;
                        updateDisplayFormats();
                    }
                    
                    function selectTimeZone(zone) {
                        timeZone = zone;
                    }
                    
                    function selectNTPServer(server) {
                        ntpServer = server;
                    }
                    
                    function updateDisplayFormats() {
                        const timeFormat12 = document.getElementById("time-format-12");
                        const timeFormat24 = document.getElementById("time-format-24");
                        const dateFormatSelect = document.getElementById("date-format-select");
                        
                        if (timeDisplayFormat === "12 hours") {
                            timeFormat12.classList.add("selected");
                            timeFormat24.classList.remove("selected");
                        } else {
                            timeFormat12.classList.remove("selected");
                            timeFormat24.classList.add("selected");
                        }
                        
                        for (let i = 0; i < dateFormatSelect.options.length; i++) {
                            const option = dateFormatSelect.options[i];
                            
                            if (option.value === dateDisplayFormat) {
                                option.selected = true;
                            } else {
                                option.selected = false;
                            }
                        }
                    }
					
					 function changeLanguage(language) {
                        const languageButtons = document.getElementsByClassName("language-button");

                        for (let i = 0; i < languageButtons.length; i++) {
                            const button = languageButtons[i];
                            button.classList.remove("selected");
                        }

                        if (language === "en") {
                            document.getElementById("button-en").classList.add("selected");
                            translateToEnglish();
                        } else if (language === "ru") {
                            document.getElementById("button-ru").classList.add("selected");
                            translateToRussian();
                        }
                    }

                    function translateToEnglish() {
                        const menuNames = [
                            "Main",
                            "Date and Time",
                            "Display",
                            "LED Control",
                            "Sound",
                            "SNMP Settings",
                            "Open VPN Settings",
                            "Language",
                            "Network",
                            "SIP Account"
                        ];
                      
                        const menuButtons = document.getElementsByClassName("menu-button");
                        
						const timeFormatLabel = document.querySelector(".time-format-label");
        const dateFormatLabel = document.querySelector(".time-format-container:nth-child(3) .time-format-label");
        const timeZoneLabel = document.querySelector(".time-format-container:nth-child(4) .time-format-label");
        const ntpServerLabel = document.querySelector(".time-format-container:nth-child(5) .time-format-label");
        const quitButton = document.querySelector(".top-button");
		const menuTitle = document.querySelector(".main-section h2");
		
    
	    menuTitle.textContent = "Date and time";
        timeFormatLabel.textContent = "Time Display Format:";
        dateFormatLabel.textContent = "Date Display Format:";
        timeZoneLabel.textContent = "Time Zone:";
        ntpServerLabel.textContent = "NTP Server:";
        quitButton.textContent = "Quit";
						
                        for (let i = 0; i < menuButtons.length; i++) {
                            const button = menuButtons[i];
                            button.textContent = menuNames[i];
                        }
                    }

                    function translateToRussian() {
                        const menuNames = [
                            "Главная",
                            "Дата и время",
                            "Дисплей",
                            "Управление LED",
                            "Звук",
                            "Настройки SNMP",
                            "Настройки Open VPN",
                            "Язык",
                            "Сеть",
                            "SIP Аккаунт"
                        ];

                        const menuButtons = document.getElementsByClassName("menu-button");

        const timeFormatLabel = document.querySelector(".time-format-label");
        const dateFormatLabel = document.querySelector(".time-format-container:nth-child(3) .time-format-label");
        const timeZoneLabel = document.querySelector(".time-format-container:nth-child(4) .time-format-label");
        const ntpServerLabel = document.querySelector(".time-format-container:nth-child(5) .time-format-label");
        const quitButton = document.querySelector(".top-button");
		const menuTitle = document.querySelector(".main-section h2");
		
        menuTitle.textContent = "Дата и время";
        timeFormatLabel.textContent = "Формат отображения времени:";
        dateFormatLabel.textContent = "Формат отображения даты:";
        timeZoneLabel.textContent = "Часовой пояс:";
        ntpServerLabel.textContent = "NTP-сервер:";
        quitButton.textContent = "Выход";

                        for (let i = 0; i < menuButtons.length; i++) {
                            const button = menuButtons[i];
                            button.textContent = menuNames[i];
                        }
                    }
                    
                    document.addEventListener('keydown', handleKeyPress);
                    updateMenu();
                    updateDisplayFormats();
                </script>
            </head>
            <body>
                <div class="top-panel">
                    <button class="top-button" onclick="goToPage('/')">Quit</button>
					<div>
                        <button id="button-en" class="language-button selected" onclick="changeLanguage('en')">EN</button>
                        <button id="button-ru" class="language-button" onclick="changeLanguage('ru')">RU</button>
                    </div>
                </div>
                <div class="container">
                    <div class="left-section">
                        <div id="menu"></div>
                       
                    </div>
                    <div class="main-section">
                        <h2>Date and time</h2>
                        <div class="time-format-container">
                            <div class="time-format-label">Time Display Format:</div>
                            <div class="time-format-option" id="time-format-12" onclick="toggleTimeDisplayFormat('12 hours')">
                                <div class="circle"></div>
                                12 hours
                            </div>
                            <div class="time-format-option" id="time-format-24" onclick="toggleTimeDisplayFormat('24 hours')">
                                <div class="circle"></div>
                                24 hours
                            </div>
                        </div>
                        <div class="time-format-container">
                            <div class="time-format-label">Date Display Format:</div>
                            <select id="date-format-select" onchange="selectDateDisplayFormat(this.value)">
                                <option value="yyyy-mm-dd">yyyy-mm-dd</option>
                                <option value="mm-dd-yyyy">mm-dd-yyyy</option>
                                <option value="dd-mm-yyyy">dd-mm-yyyy</option>
                            </select>
                        </div>
                        <div class="time-format-container">
                            <div class="time-format-label">Time Zone:</div>
                            <select id="time-zone-select" onchange="selectTimeZone(this.value)">
                                <option value="UTC -12">UTC -12</option>
                                <option value="UTC -11">UTC -11</option>
                                <option value="UTC -10">UTC -10</option>
                                <option value="UTC -9">UTC -9</option>
                                <option value="UTC -8">UTC -8</option>
                                <option value="UTC -7">UTC -7</option>
                                <option value="UTC -6">UTC -6</option>
                                <option value="UTC -5">UTC -5</option>
                                <option value="UTC -4">UTC -4</option>
                                <option value="UTC -3">UTC -3</option>
                                <option value="UTC -2">UTC -2</option>
                                <option value="UTC -1">UTC -1</option>
                                <option value="UTC 0">UTC 0</option>
                                <option value="UTC +1">UTC +1</option>
                                <option value="UTC +2">UTC +2</option>
                                <option value="UTC +3">UTC +3</option>
                                <option value="UTC +4">UTC +4</option>
                                <option value="UTC +5">UTC +5</option>
                                <option value="UTC +6">UTC +6</option>
                                <option value="UTC +7">UTC +7</option>
                                <option value="UTC +8">UTC +8</option>
                                <option value="UTC +9">UTC +9</option>
                                <option value="UTC +10">UTC +10</option>
                                <option value="UTC +11">UTC +11</option>
                                <option value="UTC +12">UTC +12</option>
                            </select>
                        </div>
                        <div class="time-format-container">
                            <div class="time-format-label">NTP Server:</div>
                            <select id="ntp-server-select" onchange="selectNTPServer(this.value)">
                                <option value="ntp0.ntp-servers.net">ntp0.ntp-servers.net</option>
                                <option value="ntp1.ntp-servers.net">ntp1.ntp-servers.net</option>
                                <option value="ntp2.ntp-servers.net">ntp2.ntp-servers.net</option>
                                <option value="ntp3.ntp-servers.net">ntp3.ntp-servers.net</option>
                                <option value="ntp4.ntp-servers.net">ntp4.ntp-servers.net</option>
                                <option value="ntp5.ntp-servers.net">ntp5.ntp-servers.net</option>
                                <option value="ntp6.ntp-servers.net">ntp6.ntp-servers.net</option>
                                <option value="ntp7.ntp-servers.net">ntp7.ntp-servers.net</option>
                            </select>
                        </div>
						<button onclick="saveSettings()">Save</button> 
                    </div>
                </div>
                <script>
                    updateMenu();
                    updateDisplayFormats();
                </script>
            </body>
        </html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}








#[get("/Display")]
async fn submenu2() -> impl Responder {
    let html = r#"
        <html>
            <head>
                <title>Display</title>
                <style>
                    body {
                        background-color: lavender;
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        justify-content: center;
                        height: 100vh;
                        margin: 0;
                    }
                     .range-warning {
                     font-size: 12px;
                     color: red;
                      margin-left: 5px;
                    }

                     .value-input-container {
                     display: flex;
                     align-items: center;
                    }

                    .arrow {
                        display: inline-block;
                        width: 0;
                        height: 0;
                        margin-left: 5px;
                        border-left: 5px solid transparent;
                        border-right: 5px solid transparent;
                    }
                    
                    .arrow-up {
                        border-top: 5px solid black;
                    }
                    
                    .arrow-down {
                        border-bottom: 5px solid black;
                    }
                    
                    .selected {
                        font-weight: bold;
                    }
                </style>
                <script>
                    let currentOption = 1;
                    let valueMenuVisible = false;
                    
                    let currentValueIndex1 = 0;
                    let currentValueIndex2 = 0;
                    let currentValueIndex3 = 0;
                    let currentValueIndex4 = 0;
                    let currentValueIndex5 = 0;
                    
                    function handleKeyPress(event) {
                        if (event.key === "ArrowDown") {
                            event.preventDefault();
                            if (valueMenuVisible) {
                                selectNextValue();
                            } else {
                                selectNextOption();
                            }
                        } else if (event.key === "ArrowUp") {
                            event.preventDefault();
                            if (valueMenuVisible) {
                                selectPreviousValue();
                            } else {
                                selectPreviousOption();
                            }
                        } else if (event.key === "Enter") {
                            event.preventDefault();
                            toggleValueMenu(currentOption);
                        }
                    }
                    
                    function selectNextOption() {
                        currentOption = (currentOption % 5) + 1;
                        valueMenuVisible = false;
                        updateMenu();
                    }
                    
                    function selectPreviousOption() {
                        currentOption = (currentOption - 2 + 5) % 5 + 1;
                        valueMenuVisible = false;
                        updateMenu();
                    }
                    
                    function selectNextValue() {
                        let valuesCount;
                        switch (currentOption) {
                            case 1:
                                valuesCount = optionValues1.length;
                                currentValueIndex1 = (currentValueIndex1 + 1) % valuesCount;
                                break;
                            case 2:
                                valuesCount = optionValues2.length;
                                currentValueIndex2 = (currentValueIndex2 + 1) % valuesCount;
                                break;
                            case 3:
                                valuesCount = optionValues3.length;
                                currentValueIndex3 = (currentValueIndex3 + 1) % valuesCount;
                                break;
                            case 4:
                                valuesCount = optionValues4.length;
                                currentValueIndex4 = (currentValueIndex4 + 1) % valuesCount;
                                break;
                            case 5:
                                valuesCount = optionValues5.length;
                                currentValueIndex5 = (currentValueIndex5 + 1) % valuesCount;
                                break;
                        }
                        updateMenu();
                    }
                    
                    function selectPreviousValue() {
                        let valuesCount;
                        switch (currentOption) {
                            case 1:
                                valuesCount = optionValues1.length;
                                currentValueIndex1 = (currentValueIndex1 - 1 + valuesCount) % valuesCount;
                                break;
                            case 2:
                                valuesCount = optionValues2.length;
                                currentValueIndex2 = (currentValueIndex2 - 1 + valuesCount) % valuesCount;
                                break;
                            case 3:
                                valuesCount = optionValues3.length;
                                currentValueIndex3 = (currentValueIndex3 - 1 + valuesCount) % valuesCount;
                                break;
                            case 4:
                                valuesCount = optionValues4.length;
                                currentValueIndex4 = (currentValueIndex4 - 1 + valuesCount) % valuesCount;
                                break;
                            case 5:
                                valuesCount = optionValues5.length;
                                currentValueIndex5 = (currentValueIndex5 - 1 + valuesCount) % valuesCount;
                                break;
                        }
                        updateMenu();
                    }
                    
                    function toggleValueMenu(optionIndex) {
                        const valueMenuDiv = document.getElementsByClassName("value-menu")[optionIndex - 1];
                        valueMenuVisible = !valueMenuVisible;
                        
                        if (valueMenuVisible) {
                            valueMenuDiv.style.display = "block";
                            valueMenuDiv.focus();
                        } else {
                            valueMenuDiv.style.display = "none";
                        }
                    }
                    
                    function selectValue(optionIndex, valueIndex) {
                        switch (optionIndex) {
                            case 1:
                                currentValueIndex1 = valueIndex;
                                break;
                            case 2:
                                currentValueIndex2 = valueIndex;
                                break;
                            case 3:
                                currentValueIndex3 = valueIndex;
                                break;
                            case 4:
                                currentValueIndex4 = valueIndex;
                                break;
                            case 5:
                                currentValueIndex5 = valueIndex;
                                break;
                        }
                        valueMenuVisible = false;
                        updateMenu();
                    }
                    
                    function updateMenu() {
                        const menuDiv = document.getElementById("menu");
                        menuDiv.innerHTML = "";
                        
                        const options = [
                            "Backlight Brightness: Active",
                            "Backlight Brightness: Idle",
                            "Active Backlight Timeout",
                            "LCD Contrast",
                            "Disable Missed Call Backlight"
                        ];
                        
                        for (let i = 1; i <= 5; i++) {
                            const optionDiv = document.createElement("div");
                            optionDiv.textContent = options[i - 1];
                            optionDiv.classList.add("menu-option");
                            
                            if (i === currentOption) {
                                optionDiv.classList.add("selected");
                            }
                            
                            const valueDiv = document.createElement("div");
                            let value;
                            switch (i) {
                                case 1:
                                    value = optionValues1[currentValueIndex1];
                                    break;
                                case 2:
                                    value = optionValues2[currentValueIndex2];
                                    break;
                                case 3:
                                    value = optionValues3[currentValueIndex3];
                                    break;
                                case 4:
                                    value = optionValues4[currentValueIndex4];
                                    break;
                                case 5:
                                    value = optionValues5[currentValueIndex5];
                                    break;
                            }
                            
                            if (i === 3) {
    // Active Backlight Timeout
const valueInputContainer = document.createElement("div");
valueInputContainer.classList.add("value-input-container");

const valueInput = document.createElement("input");
valueInput.type = "number";
valueInput.min = "0";
valueInput.max = "90"; // Set the max value to 90
valueInput.value = value;
valueInput.classList.add("value-input");
valueInputContainer.appendChild(valueInput);

const rangeWarning = document.createElement("span");
rangeWarning.textContent = "(Range: 0-90)"; // Display the range warning message
rangeWarning.classList.add("range-warning");
valueInputContainer.appendChild(rangeWarning);

valueDiv.appendChild(valueInputContainer);





const inputContainer = document.createElement("div");
inputContainer.classList.add("value-input-container");
inputContainer.appendChild(valueInput);
inputContainer.appendChild(rangeWarning);


 valueInput.addEventListener("keydown", (event) => {
    const key = event.key;
    const newValue = event.target.value;

    if (key === "Enter") {
        // End the input when Enter key is pressed
        event.preventDefault();
        valueInput.blur(); // Remove focus from the input
        selectValue(i, newValue); // Update the selected value
        toggleValueMenu(i + 1); // Toggle the value menu of the next option
        return;
    }

    if (key === "ArrowDown" || key === "ArrowUp") {
        // Ignore ArrowDown and ArrowUp keys
        event.preventDefault();
        return;
    }

    if (key.length === 1 && /\D/.test(key)) {
        // Ignore non-digit characters
        event.preventDefault();
        return;
    }

    if (key === "Backspace" || key === "Delete") {
        // Allow Backspace and Delete keys
        return;
    }

    if (newValue.length >= 2) {
        // Limit the input to two characters
        event.preventDefault();
        return;
    }
});



    valueDiv.appendChild(valueInput);
}
 else {
                                valueDiv.textContent = value;
                                valueDiv.classList.add("value");
                                
                                if (i === currentOption) {
                                    valueDiv.addEventListener("click", () => toggleValueMenu(i));
                                }
                            }
                            
                            optionDiv.appendChild(valueDiv);
                            menuDiv.appendChild(optionDiv);
                            
                            const valueMenuDiv = document.createElement("div");
                            valueMenuDiv.classList.add("value-menu");
                            
                            if (valueMenuVisible && i === currentOption) {
                                valueMenuDiv.style.display = "block";
                                valueMenuDiv.setAttribute("tabindex", "0");
                                valueMenuDiv.focus();
                            } else {
                                valueMenuDiv.style.display = "none";
                                valueMenuDiv.removeAttribute("tabindex");
                            }
                            
                            let optionValues;
                            switch (i) {
                                case 1:
                                    optionValues = optionValues1;
                                    break;
                                case 2:
                                    optionValues = optionValues2;
                                    break;
                                case 3:
                                    optionValues = optionValues3;
                                    break;
                                case 4:
                                    optionValues = optionValues4;
                                    break;
                                case 5:
                                    optionValues = optionValues5;
                                    break;
                            }
                            
                            for (let j = 0; j < optionValues.length; j++) {
                                const valueOptionDiv = document.createElement("div");
                                valueOptionDiv.textContent = optionValues[j];
                                valueOptionDiv.classList.add("value-option");
                                
                                if (j === currentValueIndex1 && i === 1 ||
                                    j === currentValueIndex2 && i === 2 ||
                                    j === currentValueIndex3 && i === 3 ||
                                    j === currentValueIndex4 && i === 4 ||
                                    j === currentValueIndex5 && i === 5) {
                                    valueOptionDiv.classList.add("selected");
                                }
                                
                                valueOptionDiv.addEventListener("click", () => selectValue(i, j));
                                
                                valueMenuDiv.appendChild(valueOptionDiv);
                            }
                            
                            menuDiv.appendChild(valueMenuDiv);
                        }
                    }
                    
                    function saveSettings() {
                        // Implement the saving logic here
                        const selectedOptions = [
                            optionValues1[currentValueIndex1],
                            optionValues2[currentValueIndex2],
                            optionValues3[currentValueIndex3],
                            optionValues4[currentValueIndex4],
                            optionValues5[currentValueIndex5]
                        ];
                        console.log("Selected options:", selectedOptions);
                    }
                    
                    function goToMainMenu() {
                        // Add your logic for going to the main menu here
                    }
                    
                    document.addEventListener("keydown", handleKeyPress);
                    const optionValues1 = ["0", "1", "2", "3", "4", "5", "6", "7", "8"];
                    const optionValues2 = ["0", "1", "2", "3", "4", "5", "6", "7", "8"];
                    const optionValues3 = Array.from({length: 91}, (_, i) => i.toString());
                    const optionValues4 = ["1", "2", "3", "4", "5"];
                    const optionValues5 = ["Yes", "No"];
                    
					
					const thirdOptionDiv = document.createElement("div");
                thirdOptionDiv.textContent = options[2];
                thirdOptionDiv.classList.add("menu-option");
                const rangeWarning = document.createElement("span");
                rangeWarning.textContent = "(0-90)";
                rangeWarning.classList.add("range-warning");
                thirdOptionDiv.appendChild(rangeWarning);
                menuDiv.appendChild(thirdOptionDiv);
                    updateMenu();
                </script>
            </head>
            <body>
                <h1>Display</h1>
                <div id="menu"></div>
                <button onclick="goToMainMenu()">Back to Main Menu</button>
                <button onclick="saveSettings()">Save</button>
                <button id="okButton" onclick="handleOkClick()">Ok</button>
                <script>
                    updateMenu();
                    
                    function handleOkClick() {
                        const enterKeyEvent = new KeyboardEvent("keydown", {
                            key: "Enter"
                        });
                        handleKeyPress(enterKeyEvent);
                    }
                    
                    function goToMainMenu() {
                        window.location.href = "/main_menu";
                    }
                    
                    function saveSettings() {
                        alert("Settings saved!");
                    }
                </script>
            </body>
        </html>
    "#;

    HttpResponse::Ok().body(html)
}










#[get("/submenu3")]
async fn submenu3() -> impl Responder {
    let html = r#"
        <html>
            <head>
                <title>Submenu 3</title>
                <style>
                    .arrow {
                        display: inline-block;
                        width: 0;
                        height: 0;
                        margin-left: 5px;
                        border-top: 5px solid transparent;
                        border-bottom: 5px solid transparent;
                    }
                    
                    .arrow-right {
                        border-left: 5px solid black;
                    }
                    
                    .selected {
                        font-weight: bold;
                    }
                    
                    .option-row {
                        display: flex;
                        align-items: center;
                    }
                    
                    .option-label {
                        margin-right: 10px;
                    }
                    
                    .option-value {
                        padding: 5px 10px;
                        border: 1px solid #ccc;
                        cursor: pointer;
                    }
                    
                    .option-value.selected {
                        background-color: #e0e0e0;
                    }
                    
                    .option-list {
                        display: none;
                        position: absolute;
                        background-color: #fff;
                        border: 1px solid #ccc;
                        padding: 5px;
                        max-height: 100px;
                        overflow-y: auto;
                    }
                    
                    .option-list.show {
                        display: block;
                    }
                    
                    .option-list-item {
                        cursor: pointer;
                        padding: 3px;
                    }
                </style>
                <script>
                    let currentOption = 1;
                    
                    function handleKeyPress(event) {
                        if (event.key === "ArrowDown") {
                            event.preventDefault();
                            goToNextOption();
                        } else if (event.key === "ArrowUp") {
                            event.preventDefault();
                            goToPreviousOption();
                        } else if (event.key === "Enter") {
                            event.preventDefault();
                            goToMainMenu();
                        }
                    }
                    
                    function goToNextOption() {
                        if (currentOption < 9) {
                            currentOption++;
                        }
                        updateMenu();
                    }
                    
                    function goToPreviousOption() {
                        if (currentOption > 1) {
                            currentOption--;
                        }
                        updateMenu();
                    }
                    
                    function goToMainMenu() {
                        window.location.href = "/main_menu";
                    }
                    
                    function toggleOptionValue() {
                        const optionValue = document.getElementById("option-value");
                        const optionList = document.getElementById("option-list");
                        optionValue.classList.toggle("selected");
                        optionList.classList.toggle("show");
                    }
                    
                    function selectOption(option) {
                        const optionValue = document.getElementById("option-value");
                        optionValue.textContent = option;
                        toggleOptionValue();
                    }
                    
                    function updateMenu() {
                        const menuDiv = document.getElementById("menu");
                        menuDiv.innerHTML = "";
                        
                        const optionRow = document.createElement("div");
                        optionRow.classList.add("option-row");
                        
                        const optionLabel = document.createElement("div");
                        optionLabel.classList.add("option-label");
                        optionLabel.textContent = "Option:";
                        
                        const optionValue = document.createElement("div");
                        optionValue.classList.add("option-value");
                        optionValue.id = "option-value";
                        optionValue.textContent = "No";
                        optionValue.addEventListener("click", toggleOptionValue);
                        
                        const optionList = document.createElement("div");
                        optionList.classList.add("option-list");
                        optionList.id = "option-list";
                        
                        const optionListRow1 = document.createElement("div");
                        optionListRow1.classList.add("option-list-item");
                        
                        const optionListItemYes = document.createElement("div");
                        optionListItemYes.textContent = "Yes";
                        optionListItemYes.addEventListener("click", () => selectOption("Yes"));
                        optionListRow1.appendChild(optionListItemYes);
                        
                        const optionListRow2 = document.createElement("div");
                        optionListRow2.classList.add("option-list-item");
                        
                        const optionListItemNo = document.createElement("div");
                        optionListItemNo.textContent = "No";
                        optionListItemNo.addEventListener("click", () => selectOption("No"));
                        optionListRow2.appendChild(optionListItemNo);
                        
                        optionList.appendChild(optionListRow1);
                        optionList.appendChild(optionListRow2);
                        
                        if (currentOption === 1) {
                            optionValue.classList.add("selected");
                        }
                        
                        optionRow.appendChild(optionLabel);
                        optionRow.appendChild(optionValue);
                        optionRow.appendChild(optionList);
                        
                        menuDiv.appendChild(optionRow);
                    }
                    
                    document.addEventListener('keydown', handleKeyPress);
                    updateMenu();
                </script>
            </head>
            <body>
                <h1>Submenu 3</h1>
                <div id="menu"></div>
                <button onclick="goToMainMenu()">Back to Main Menu</button>
                <script>
                    updateMenu();
                    document.addEventListener('keydown', handleKeyPress);
                </script>
            </body>
        </html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}


#[get("/submenu4")]
async fn submenu4() -> impl Responder {
    let html = r#"
        <html>
            <head>
                <title>Submenu 4</title>
                <style>
                    .arrow {
                        display: inline-block;
                        width: 0;
                        height: 0;
                        margin-left: 5px;
                        border-top: 5px solid transparent;
                        border-bottom: 5px solid transparent;
                    }
                    
                    .arrow-right {
                        border-left: 5px solid black;
                    }
                    
                    .selected {
                        font-weight: bold;
                    }
                </style>
                <script>
                    let currentOption = 1;
                    
                    function handleKeyPress(event) {
                        if (event.key === "ArrowDown") {
                            event.preventDefault();
                            goToNextOption();
                        } else if (event.key === "ArrowUp") {
                            event.preventDefault();
                            goToPreviousOption();
                        } else if (event.key === "Enter") {
                            event.preventDefault();
                            goToMainMenu();
                        }
                    }
                    
                    function goToNextOption() {
                        if (currentOption < 9) {
                            currentOption++;
                        }
                        updateMenu();
                    }
                    
                    function goToPreviousOption() {
                        if (currentOption > 1) {
                            currentOption--;
                        }
                        updateMenu();
                    }
                    
                    function goToMainMenu() {
                        window.location.href = "/main_menu";
                    }
                    
                    function updateMenu() {
                        const menuDiv = document.getElementById("menu");
                        menuDiv.innerHTML = "";
                        
                        for (let i = 1; i <= 9; i++) {
                            const optionDiv = document.createElement("div");
                            optionDiv.textContent = "Submenu 4 Option " + i;
                            
                            if (i === currentOption) {
                                optionDiv.classList.add("selected");
                            }
                            
                            menuDiv.appendChild(optionDiv);
                        }
                    }
                    
                    document.addEventListener('keydown', handleKeyPress);
                    updateMenu();
                </script>
            </head>
            <body>
                <h1>Submenu 4</h1>
                <div id="menu"></div>
                <button onclick="goToMainMenu()">Back to Main Menu</button>
					 <script>
                     updateMenu();
                     document.addEventListener('keydown', handleKeyPress);
                     </script>
            </body>
        </html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

#[get("/submenu5")]
async fn submenu5() -> impl Responder {
    let html = r#"
        <html>
            <head>
                <title>Submenu 5</title>
                <style>
                    .arrow {
                        display: inline-block;
                        width: 0;
                        height: 0;
                        margin-left: 5px;
                        border-top: 5px solid transparent;
                        border-bottom: 5px solid transparent;
                    }
                    
                    .arrow-right {
                        border-left: 5px solid black;
                    }
                    
                    .selected {
                        font-weight: bold;
                    }
                </style>
                <script>
                    let currentOption = 1;
                    
                    function handleKeyPress(event) {
                        if (event.key === "ArrowDown") {
                            event.preventDefault();
                            goToNextOption();
                        } else if (event.key === "ArrowUp") {
                            event.preventDefault();
                            goToPreviousOption();
                        } else if (event.key === "Enter") {
                            event.preventDefault();
                            goToMainMenu();
                        }
                    }
                    
                    function goToNextOption() {
                        if (currentOption < 9) {
                            currentOption++;
                        }
                        updateMenu();
                    }
                    
                    function goToPreviousOption() {
                        if (currentOption > 1) {
                            currentOption--;
                        }
                        updateMenu();
                    }
                    
                    function goToMainMenu() {
                        window.location.href = "/main_menu";
                    }
                    
                    function updateMenu() {
                        const menuDiv = document.getElementById("menu");
                        menuDiv.innerHTML = "";
                        
                        for (let i = 1; i <= 9; i++) {
                            const optionDiv = document.createElement("div");
                            optionDiv.textContent = "Submenu 5 Option " + i;
                            
                            if (i === currentOption) {
                                optionDiv.classList.add("selected");
                            }
                            
                            menuDiv.appendChild(optionDiv);
                        }
                    }
                    
                    document.addEventListener('keydown', handleKeyPress);
                    updateMenu();
                </script>
            </head>
            <body>
                <h1>Submenu 5</h1>
                <div id="menu"></div>
                <button onclick="goToMainMenu()">Back to Main Menu</button>
					 <script>
                     updateMenu();
                     document.addEventListener('keydown', handleKeyPress);
                     </script>
            </body>
        </html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}


#[get("/submenu6")]
async fn submenu6() -> impl Responder {
    let html = r#"
        <html>
            <head>
                <title>Submenu 6</title>
                <style>
                    .arrow {
                        display: inline-block;
                        width: 0;
                        height: 0;
                        margin-left: 5px;
                        border-top: 5px solid transparent;
                        border-bottom: 5px solid transparent;
                    }
                    
                    .arrow-right {
                        border-left: 5px solid black;
                    }
                    
                    .selected {
                        font-weight: bold;
                    }
                </style>
                <script>
                    let currentOption = 1;
                    
                    function handleKeyPress(event) {
                        if (event.key === "ArrowDown") {
                            event.preventDefault();
                            goToNextOption();
                        } else if (event.key === "ArrowUp") {
                            event.preventDefault();
                            goToPreviousOption();
                        } else if (event.key === "Enter") {
                            event.preventDefault();
                            goToMainMenu();
                        }
                    }
                    
                    function goToNextOption() {
                        if (currentOption < 9) {
                            currentOption++;
                        }
                        updateMenu();
                    }
                    
                    function goToPreviousOption() {
                        if (currentOption > 1) {
                            currentOption--;
                        }
                        updateMenu();
                    }
                    
                    function goToMainMenu() {
                        window.location.href = "/main_menu";
                    }
                    
                    function updateMenu() {
                        const menuDiv = document.getElementById("menu");
                        menuDiv.innerHTML = "";
                        
                        for (let i = 1; i <= 9; i++) {
                            const optionDiv = document.createElement("div");
                            optionDiv.textContent = "Submenu 6 Option " + i;
                            
                            if (i === currentOption) {
                                optionDiv.classList.add("selected");
                            }
                            
                            menuDiv.appendChild(optionDiv);
                        }
                    }
                    
                    document.addEventListener('keydown', handleKeyPress);
                    updateMenu();
                </script>
            </head>
            <body>
                <h1>Submenu 6</h1>
                <div id="menu"></div>
                <button onclick="goToMainMenu()">Back to Main Menu</button>
					 <script>
                     updateMenu();
                     document.addEventListener('keydown', handleKeyPress);
                     </script>
            </body>
        </html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}


#[get("/submenu7")]
async fn submenu7() -> impl Responder {
    let html = r#"
        <html>
            <head>
                <title>Submenu 7</title>
                <style>
                    .arrow {
                        display: inline-block;
                        width: 0;
                        height: 0;
                        margin-left: 5px;
                        border-top: 5px solid transparent;
                        border-bottom: 5px solid transparent;
                    }
                    
                    .arrow-right {
                        border-left: 5px solid black;
                    }
                    
                    .selected {
                        font-weight: bold;
                    }
                </style>
                <script>
                    let currentOption = 1;
                    
                    function handleKeyPress(event) {
                        if (event.key === "ArrowDown") {
                            event.preventDefault();
                            goToNextOption();
                        } else if (event.key === "ArrowUp") {
                            event.preventDefault();
                            goToPreviousOption();
                        } else if (event.key === "Enter") {
                            event.preventDefault();
                            goToMainMenu();
                        }
                    }
                    
                    function goToNextOption() {
                        if (currentOption < 9) {
                            currentOption++;
                        }
                        updateMenu();
                    }
                    
                    function goToPreviousOption() {
                        if (currentOption > 1) {
                            currentOption--;
                        }
                        updateMenu();
                    }
                    
                    function goToMainMenu() {
                        window.location.href = "/main_menu";
                    }
                    
                    function updateMenu() {
                        const menuDiv = document.getElementById("menu");
                        menuDiv.innerHTML = "";
                        
                        for (let i = 1; i <= 9; i++) {
                            const optionDiv = document.createElement("div");
                            optionDiv.textContent = "Submenu 7 Option " + i;
                            
                            if (i === currentOption) {
                                optionDiv.classList.add("selected");
                            }
                            
                            menuDiv.appendChild(optionDiv);
                        }
                    }
                    
                    document.addEventListener('keydown', handleKeyPress);
                    updateMenu();
                </script>
            </head>
            <body>
                <h1>Submenu 7</h1>
                <div id="menu"></div>
                <button onclick="goToMainMenu()">Back to Main Menu</button>
					 <script>
                     updateMenu();
                     document.addEventListener('keydown', handleKeyPress);
                     </script>
            </body>
        </html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}


#[get("/submenu8")]
async fn submenu8() -> impl Responder {
    let html = r#"
        <html>
            <head>
                <title>Submenu 8</title>
                <style>
                    .arrow {
                        display: inline-block;
                        width: 0;
                        height: 0;
                        margin-left: 5px;
                        border-top: 5px solid transparent;
                        border-bottom: 5px solid transparent;
                    }
                    
                    .arrow-right {
                        border-left: 5px solid black;
                    }
                    
                    .selected {
                        font-weight: bold;
                    }
                </style>
                <script>
                    let currentOption = 1;
                    
                    function handleKeyPress(event) {
                        if (event.key === "ArrowDown") {
                            event.preventDefault();
                            goToNextOption();
                        } else if (event.key === "ArrowUp") {
                            event.preventDefault();
                            goToPreviousOption();
                        } else if (event.key === "Enter") {
                            event.preventDefault();
                            goToMainMenu();
                        }
                    }
                    
                    function goToNextOption() {
                        if (currentOption < 9) {
                            currentOption++;
                        }
                        updateMenu();
                    }
                    
                    function goToPreviousOption() {
                        if (currentOption > 1) {
                            currentOption--;
                        }
                        updateMenu();
                    }
                    
                    function goToMainMenu() {
                        window.location.href = "/main_menu";
                    }
                    
                    function updateMenu() {
                        const menuDiv = document.getElementById("menu");
                        menuDiv.innerHTML = "";
                        
                        for (let i = 1; i <= 9; i++) {
                            const optionDiv = document.createElement("div");
                            optionDiv.textContent = "Submenu 8 Option " + i;
                            
                            if (i === currentOption) {
                                optionDiv.classList.add("selected");
                            }
                            
                            menuDiv.appendChild(optionDiv);
                        }
                    }
                    
                    document.addEventListener('keydown', handleKeyPress);
                    updateMenu();
                </script>
            </head>
            <body>
                <h1>Submenu 8</h1>
                <div id="menu"></div>
                <button onclick="goToMainMenu()">Back to Main Menu</button>
					 <script>
                     updateMenu();
                     document.addEventListener('keydown', handleKeyPress);
                     </script>
            </body>
        </html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}


#[get("/submenu9")]
async fn submenu9() -> impl Responder {
    let html = r#"
        <html>
            <head>
                <title>Submenu 9</title>
                <style>
                    .arrow {
                        display: inline-block;
                        width: 0;
                        height: 0;
                        margin-left: 5px;
                        border-top: 5px solid transparent;
                        border-bottom: 5px solid transparent;
                    }
                    
                    .arrow-right {
                        border-left: 5px solid black;
                    }
                    
                    .selected {
                        font-weight: bold;
                    }
                </style>
                <script>
                    let currentOption = 1;
                    
                    function handleKeyPress(event) {
                        if (event.key === "ArrowDown") {
                            event.preventDefault();
                            goToNextOption();
                        } else if (event.key === "ArrowUp") {
                            event.preventDefault();
                            goToPreviousOption();
                        } else if (event.key === "Enter") {
                            event.preventDefault();
                            goToMainMenu();
                        }
                    }
                    
                    function goToNextOption() {
                        if (currentOption < 9) {
                            currentOption++;
                        }
                        updateMenu();
                    }
                    
                    function goToPreviousOption() {
                        if (currentOption > 1) {
                            currentOption--;
                        }
                        updateMenu();
                    }
                    
                    function goToMainMenu() {
                        window.location.href = "/main_menu";
                    }
                    
                    function updateMenu() {
                        const menuDiv = document.getElementById("menu");
                        menuDiv.innerHTML = "";
                        
                        for (let i = 1; i <= 9; i++) {
                            const optionDiv = document.createElement("div");
                            optionDiv.textContent = "Submenu 9 Option " + i;
                            
                            if (i === currentOption) {
                                optionDiv.classList.add("selected");
                            }
                            
                            menuDiv.appendChild(optionDiv);
                        }
                    }
                    
                    document.addEventListener('keydown', handleKeyPress);
                    updateMenu();
                </script>
            </head>
            <body>
                <h1>Submenu 9</h1>
                <div id="menu"></div>
                <button onclick="goToMainMenu()">Back to Main Menu</button>
					 <script>
                     updateMenu();
                     document.addEventListener('keydown', handleKeyPress);
                     </script>
            </body>
        </html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
		    .service(login)
            .service(process_login)

            .service(main_menu)
            .service(submenu1)
            .service(submenu2)
            .service(submenu3)
            .service(submenu4)
            .service(submenu5)
            .service(submenu6)
            .service(submenu7)
            .service(submenu8)
            .service(submenu9)
			
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
