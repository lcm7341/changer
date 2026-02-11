function setTextContent(element_id, text) {
  document.getElementById(element_id).textContent = text;
}

function addInnerHtml(element_id, html) {
  document.getElementById(element_id).innerHTML += html;
}

async function refresh(result) {
  if (!result || !result.bills || !result.coins) {
    console.error("bad payload:", result);
    return;
  }

  document.getElementById("bills").innerHTML = "";
  document.getElementById("coins").innerHTML = "";

  for (const [name, count] of Object.entries(result.bills)) {
    for (let i = 0; i < count; i++) {
      const html = `<img src="currencies/${name}.png" width="261" height="100">`;
      addInnerHtml("bills", html);
    }
  }

  for (const [name, count] of Object.entries(result.coins)) {
    for (let i = 0; i < count; i++) {
      let size = 50;
      if (name === "dimes" || name === "pennies") size *= 0.3;
      if (name === "nickels") size *= 0.9;

      const html = `<img src="currencies/${name}.png" width="${size}" height="${size}">`;
      addInnerHtml("coins", html);
    }
    addInnerHtml("coins", "<br>");
  }
}

document.getElementById("calculate").onclick = async () => {
  const cost = parseFloat(document.getElementById("cost_input").value);
  const given = parseFloat(document.getElementById("given_input").value);

  const res = await fetch("https://changer-iyfu.onrender.com/api/calculate_change", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ cost, given }),
  });

  if (!res.ok) {
    console.error("HTTP error:", res.status, await res.text());
    return;
  }

  const data = await res.json();
  await refresh(data);
};
