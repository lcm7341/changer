async function refresh() {
  const res = await fetch("http://localhost:3001/api/get_cost");
  if (!res.ok) throw new Error(`get_cost failed: ${res.status}`);
  const cost = await res.json();

  document.getElementById("cost").textContent = cost.cost;
  console.log(cost);
}

document.getElementById("new_cost_btn").onclick = async () => {
  try {
    const inputEl = document.getElementById("new_cost");

    const cost = parseFloat(inputEl.value);

    // If new_cost is an <input> or <textarea>, use .value
    const payload = { cost: cost};

    const res = await fetch("http://localhost:3001/api/set_cost", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    });

    if (!res.ok) {
      const text = await res.text(); // helpful if server returns non-JSON error
      throw new Error(`set_cost failed: ${res.status} ${text}`);
    }

    await refresh();
  } catch (e) {
    console.error(e);
  }
};

refresh().catch(console.error);
