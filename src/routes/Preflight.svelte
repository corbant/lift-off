<script>
	import { Checkbox } from "carbon-components-svelte";
	let checklistItems = ["Item 1", "Item 2", "Item 3"];
	let checkedItems = [];
</script>





<h1>Preflight Checklist</h1>

{#each checklistItems as item}
	<Checkbox bind:group={checkedItems} labelText={item} value={item} />
{/each}

<strong>Selected:</strong>
{JSON.stringify(checkedItems)}
