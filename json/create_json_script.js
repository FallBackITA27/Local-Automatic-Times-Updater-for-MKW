/* Launch this in the browser's console. */
/* -- FalB */

let out = [];
let out_json = {};
let x = 1;
for (let em of document.getElementsByTagName("tr")) {
	if (x==1) { x--; continue; }
	let tds = em.getElementsByTagName("td");
	let text = tds[0].children[0];
	let name = text.innerHTML.replaceAll(" ","").replace("-Normal","").replace("-No-shortcut","").replace("-Shortcut","_sc").replace("-Glitch","_g").replace("GCN","r").replace("GBA","r").replace("DS","r").replace("SNES","r").replace("N64","r").replace("Luigi","l").replace("Circuit","c").replace("MooMooMeadows","mmm").replace("MushroomGorge","mg").replace("Toad'sFactory","tf").replace("Mario","m").replace("CoconutMall").replace("DKSummit","dksc").replace("Wario'sGoldMine","wgm").replace("Daisy","d").replace("KoopaCape","kc").replace("MapleTreeway","mt").replace("GrumbleVolcano","gv").replace("DryDryRuins","ddr").replace("MoonviewHighway","mh").replace("Bowser's","b").replace("Castle","c").replace("RainbowRoad","rr").replace("Peach","p").replace("Beach","b").replace("YoshiFalls","yf").replace("GhostValley","gv").replace("Raceway","r").replace("SherbetLand","sl").replace("ShyGuyBeach","sgb").replace("DelfinoSquare","ds").replace("WaluigiStadium","ws").replace("DesertHills","dh").replace("Bowser","b").replace("DK","dk").replace("'sJungleParkway","jp").replace("Gardens","g").replace("Mountain","m");
	let href = text.href.replace(".html",".json").replace("https://www.chadsoft.co.uk/time-trials","");
	out.push([href,name]);
	out_json[href]=name;
}
console.log(JSON.stringify(out)) // Array;
console.log(JSON.stringify(out_json)) // Hashmap;