const { invoke } = window.__TAURI__.tauri;

let userData;

/**
 * Adds a popup to the popups div
 * @param {string} text - innerHTML of the popup
 * @param {string} type - `err` or `ok`. Determines color
 * @returns {null}
 */
async function addPopup(text, type) {
  let div = document.getElementById("popups_div");

  let nthPopup = document.createElement("div");
  nthPopup.classList.add("popup");
  if (type==="err") nthPopup.classList.add("err");
  if (type==="ok") nthPopup.classList.add("ok");

  let btn = document.createElement("button");
  btn.innerHTML = "×";
  btn.addEventListener("click",function(){
    btn.parentElement.remove(self);
  });

  let p = document.createElement("p");
  p.innerHTML = text;

  nthPopup.appendChild(btn);
  nthPopup.appendChild(p);
  nthPopup.appendChild(document.createElement("div"));
  // so that the text stays in the center. `justify-content: space-between;` forces the second of three elements at the center.

  div.appendChild(nthPopup);
}

let count_loading_dots = 0;
let loading_anim = setInterval(function () {
  let em = document.getElementById("loading_text");
  if (count_loading_dots === 3) {
    em.innerHTML = em.innerHTML.substring(0, em.innerHTML.length - 3);
    count_loading_dots = 0;
    return;
  }
  em.innerHTML += ".";
  count_loading_dots++;
}, 1000);


/**
 * Reads the config.cfg file from the CWD and sets the values in the Options fieldset.
 * @returns {null}
 */
async function readConfig(){
  invoke("read_config").then(r => {
    let json_data = JSON.parse(r);
    if (json_data.chadUser != undefined) document.getElementById("chadsoftInput").value = json_data.chadUser;
    if (json_data.mkwppUser != undefined) document.getElementById("mkwppInput").value = json_data.mkwppUser;
    if (json_data.mklUser != undefined) document.getElementById("mklInput").value = json_data.mklUser;
    userData = json_data;
  });
}

readConfig();
clearInterval(loading_anim);
document.getElementsByTagName("main")[0].classList.toggle("off");
document.getElementById("loading_div").classList.toggle("off");

/**
 * Generic function for saving
 * @param {string} internal_function_name - The name of the function inside of the main.rs file
 * @param {string} data - The data to be saved
 * @returns {null}
 */
async function save(internal_function_name, data) {
  invoke(internal_function_name, { data: data }).then(r => {
    if (r === "ok") {
      addPopup("Saved successfully","ok")
    } else {
      addPopup(r,"err");
    }  
  });
}

/**
 * Wrapper of `save()` for saving Chadsoft User Data
 * @returns {null}
 */
async function saveChadsoft() { save("save_chadsoft_user", document.getElementById("chadsoftInput").value) }

/**
 * Wrapper of `save()` for saving MKWPP User Data
 * @returns {null}
 */
async function saveMKWPP() { save("save_mkwpp_user", document.getElementById("mkwppInput").value) }

/**
 * Wrapper of `save()` for saving MKL User Data
 * @returns {null}
 */
async function saveMKL() { save("save_mkl_user", document.getElementById("mklInput").value) }

/**
 * Triggers all Saving functions
 * @returns {null}
 */
async function saveAllOptions(){
  saveChadsoft();
  saveMKWPP();
  saveMKL();
  readConfig();
}

document.getElementById("saveOptions").addEventListener("click",saveAllOptions);

addEventListener("keydown", function (evt) {
  if (evt.ctrlKey && evt.key === "s") {
    evt.preventDefault();
    saveAllOptions();
  }
});

document.getElementById("mkwppUpdaterButton").addEventListener("click",function(){
  addPopup("Started MKWPP Updater","ok");
  invoke("mkwpp_mode", {mkwppId: userData.mkwppUser.split("=")[1], chadsoftId: userData.chadUser.split("players/")[1].split(".")[0]}).then(r=>{
    for (let i of r) {
      if (i.contains("must") || i.contains("Sorry") || i.contains("Unknown") || i.contains("down")) {
        addPopup(i,"err");
      } else {
        addPopup(i,"ok");
      }
    }
  });
});