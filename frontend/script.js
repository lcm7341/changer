function setTextContent(element_id, text) {
  document.getElementById(element_id).textContent = text;
}

function addInnerHtml(element_id, html) {
  document.getElementById(element_id).innerHTML += html;
}

async function refresh() {
  const res = await fetch("http://localhost:3001/api/get_change");
  if (!res.ok) throw new Error(`get_cost failed: ${res.status}`);
  const result = await res.json();
  document.getElementById("bills").innerHTML = ""
  document.getElementById("coins").innerHTML = ""

  for (entry of Object.entries(result.bills)) {
    for (let i = 0; i < entry[1]; i++) {
      let html = `<img src="currencies/${entry[0]}.png" width="261" height="100">`
      addInnerHtml("bills", html)
    }
  }

  for (entry of Object.entries(result.coins)) {
    for (let i = 0; i < entry[1]; i++) {
      let size = 50;
      if (entry[0] == "dimes" || entry[0] == "pennies") size *= 0.3;
      if (entry[0] == "nickels") size *= 0.9;
      let html = `<img src="currencies/${entry[0]}.png" width="${size}" height="${size}">`
      addInnerHtml("coins", html)
    }
    let html = `<br>`
    addInnerHtml("coins", html)
  }

}

document.getElementById("calculate").onclick = async () => {
  const cost_input = document.getElementById("cost_input");
  const given_input = document.getElementById("given_input");

  const cost = parseFloat(cost_input.value);
  const given = parseFloat(given_input.value);

  const payload = { cost: cost, given: given };

  const res = await fetch("http://localhost:3001/api/calculate_change", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });

  await refresh();
};

refresh().catch(console.error);
