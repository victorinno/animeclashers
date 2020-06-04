import * as wasm from "anime-clashers";

const draw_form_old_player = () => {
  return wasm.Player.has_player();
};

const new_player = () => {
  wasm.Player.new(document.getElementById("name_new_form").value);
};

let player = null;

const renderLoop = () => {
  if (draw_form_old_player()) {
    show_player_screen();
    load_player();
    set_player_name();
    
  } else {
    document.getElementById("form_new_player").hidden = false;
    document.getElementById("player_area").hidden = true;
    document.getElementById("new_player_btn").onclick = new_player;
  }

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);

function load_player() {
  player = wasm.Player.get_player();
}

function set_player_name() {
  document.getElementById("player_name").innerText = player.name;
}

function show_player_screen() {
  document.getElementById("form_new_player").hidden = true;
  document.getElementById("player_area").hidden = false;
}
