import * as wasm from "anime-clashers";

wasm.init();

let player = null;
let curr_area = "form_new_player";
let curr_clasher = null;
let clasher_list_compiled = false;
let power_list_compiled = false;

const new_player = () => {
  player = wasm.Player.new(document.getElementById("name_new_form").value);
  curr_area = "player_area";
};

const draw_form_old_player = () => {
  return wasm.Player.has_player();
};

const show_area = () => {
  let elements = document.getElementsByClassName("area");
  for (let element of elements) {
    element.hidden = element.id != curr_area;
  }
};
const show_new_clasher_area = () => {
  curr_area = "new_clasher_area";
  console.log(curr_area);
};

const powers_area = (() => {
  const load = () => {
    if (!power_list_compiled) {
      let powers = JSON.parse(wasm.Power.load_db(curr_clasher.id));

      let list_powers = document.getElementById("list_powers");
      while (list_powers.firstChild) {
        list_powers.removeChild(list_powers.lastChild);
      }
      powers.forEach((p) => {
        let div = document.createElement("div");
        div.style.width = "100%";
        div.style.marginBottom = "5px";
        div.style.marginTop = "5px";
        div.style.height = "108px";
        div.style.borderStyle = "solid";
        let name = document.createElement("h2");
        name.style.float = "left";
        name.style.width = "33.33%";
        name.innerHTML = p.name;
        div.appendChild(name);
        let level = document.createElement("h2");
        level.style.float = "left";
        level.style.width = "33.33%";
        level.innerHTML = "Lvl. " + p.level;
        div.appendChild(level);
        let power_action = document.createElement("h2");
        power_action.style.float = "left";
        power_action.style.width = "33.33%";
        power_action.innerHTML = p.power_action;
        div.appendChild(power_action);
        let potency = document.createElement("span");
        potency.style.float = "left";
        potency.style.width = "33.33%";
        potency.innerHTML = "potency: " + p.potency;
        div.appendChild(potency);
        let precision = document.createElement("span");
        precision.style.float = "left";
        precision.style.width = "33.33%";
        precision.innerHTML = "precision: " + p.potency;
        div.appendChild(precision);
        let power_type = document.createElement("span");
        power_type.style.float = "left";
        power_type.style.width = "33.33%";
        power_type.innerHTML = "type: " + p.potency;
        div.appendChild(power_type);
        list_powers.appendChild(div);
      });
    }
    power_list_compiled = true;
  };

  const back = () => {
    document.getElementById("powers_btn_back_nav").onclick = () =>
      (curr_area = "clasher_area");
  };
  const new_power = () => {
    document.getElementById("new_powers_btn").onclick = () =>
      new_powers_area.navigate();
  };

  return {
    name: () => "powers_area",
    navigate: () => {
      curr_area = powers_area.name();
      new_powers_area.init();
      power_list_compiled = false;
    },

    print: () => {
      back();
      new_power();
      load();
    },
  };
})();

const new_powers_area = (() => {
  let types_printed = false;
  let action_printed = false;

  const actions = () => {
    let new_type_action_selection = document.getElementById(
      "new_type_action_selection"
    );
    if (!action_printed) {
      while (new_type_action_selection.firstChild) {
        new_type_action_selection.removeChild(
          new_type_action_selection.lastChild
        );
      }

      let types = JSON.parse(wasm.Power.list_types_action());
      types.forEach((t) => {
        let o = document.createElement("option");
        o.value = t;
        o.innerHTML = t;
        new_type_action_selection.appendChild(o);
      });
    }
    action_printed = true;
  };
  const types = () => {
    let new_type_power_selection = document.getElementById(
      "new_type_power_selection"
    );
    if (!types_printed) {
      while (new_type_power_selection.firstChild) {
        new_type_power_selection.removeChild(
          new_type_power_selection.lastChild
        );
      }

      let types = JSON.parse(wasm.Power.list_types_power());
      types.forEach((t) => {
        let o = document.createElement("option");
        o.value = t;
        o.innerHTML = t;
        new_type_power_selection.appendChild(o);
      });
    }
    types_printed = true;
  };
  const create = () => {
    document.getElementById("new_power_create_btn").onclick = () => {
      wasm.Power.create(
        document.getElementById("new_power_name").value,
        document.getElementById("new_level_selection").value,
        document.getElementById("new_type_power_selection").value,
        document.getElementById("new_type_action_selection").value,
        curr_clasher.id
      );
      powers_area.navigate();
    };
  };

  return {
    init: () => {
      types_printed = false;
      action_printed = false;
    },
    name: () => "new_powers_area",
    navigate: () => (curr_area = "new_powers_area"),
    print: () => {
      document.getElementById("new_powers_btn_back_nav").onclick = () =>
        (curr_area = "powers_area");
      actions();
      types();
      create();
    },
  };
})();

show_area();
if (draw_form_old_player()) {
  show_player_screen();
  load_player();
  set_player_name();
  document.getElementById("new_clasher_btn").onclick = show_new_clasher_area;
} else {
  curr_area = "form_new_player";
  document.getElementById("new_player_btn").onclick = new_player;
}
show_area();
const renderLoop = () => {
  if (curr_area == "player_area") {
    show_player_screen();
    load_player();
    set_player_name();

    if (!clasher_list_compiled) {
      let clashers = JSON.parse(wasm.Clasher.load_db());
      const ul = document.getElementById("list_of_clashers");
      while (ul.firstChild) {
        ul.removeChild(ul.lastChild);
      }
      clashers.forEach((c) => {
        let li = document.createElement("div");
        li.innerHTML = c.name + " - " + c.clazz.name + "   ";
        let b = document.createElement("button");
        b.id = "id_" + Math.random();
        b.innerHTML = "Select";

        li.appendChild(b);
        ul.appendChild(li);
        document.getElementById(b.id).onclick = () => {
          curr_clasher = wasm.Clasher.get_by_name(c.name);
          curr_area = "clasher_area";
        };
      });
      clasher_list_compiled = true;
    }

    document.getElementById("new_clasher_btn").onclick = show_new_clasher_area;
    setTimeout(function () {}, 1000);
  } else if (curr_area == "form_new_player") {
    // curr_area = "form_new_player";
    document.getElementById("new_player_btn").onclick = new_player;
  } else if (curr_area == "new_clasher_area") {
    // curr_area = "new_clasher_area";
    let classes = JSON.parse(wasm.Class.list_all_classes());
    const select = document.getElementById("new_clasher_class");
    while (select.firstChild) {
      select.removeChild(select.lastChild);
    }
    classes.forEach((classe) => {
      var option = document.createElement("option");
      option.text = classe.name;
      option.value = JSON.stringify(classe);
      select.appendChild(option);
    });

    let curr_class = JSON.parse(select.value);

    document.getElementById("power_chance").innerHTML =
      "Power %: " + curr_class.power;
    document.getElementById("dexterity_chance").innerHTML =
      "Dextterity %: " + curr_class.power;
    document.getElementById("agility_chance").innerHTML =
      "Agility %: " + curr_class.power;
    document.getElementById("resistance_chance").innerHTML =
      "Resistance %: " + curr_class.power;
    document.getElementById("hp_chance").innerHTML =
      "HP %: " + curr_class.power;
    document.getElementById("ep_chance").innerHTML =
      "EP %: " + curr_class.power;

    document.getElementById("create_clasher_btn").onclick = () => {
      let clazz = JSON.parse(
        document.getElementById("new_clasher_class").value
      );
      let c = wasm.Clasher.create_clasher(
        document.getElementById("name_new_clasher_form").value,
        wasm.Class.get_by_name(clazz.name)
      );
      curr_area = "player_area";
      clasher_list_compiled = false;
    };
  } else if (curr_area == "clasher_area") {
    document.getElementById("clasher_top_name").innerHTML = curr_clasher.name;
    document.getElementById("powers_btn_nav").onclick = () =>
      (curr_area = "powers_area");
  } else if (curr_area == powers_area.name()) {
    powers_area.print();
  } else if (curr_area == new_powers_area.name()) {
    new_powers_area.print();
  }
  show_area();
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
  curr_area = "player_area";
}
