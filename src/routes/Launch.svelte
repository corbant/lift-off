<script>
	import { Line } from 'svelte-chartjs'
</script>


<div>
	<Line data={{
		datasets: [{
			label: "Altitude",
			data: [1, 5, 1, 7, 5],
			backgroundColor: "rgba(255, 99, 132, 0.2)",
			borderColor: "rgba(255, 99, 132, 1)",
			borderWidth: 1
		}]
	}}
	options={{
		responsive: true,
		scales: {
			x: {
				ticks: {
					source: 'auto'
				}
			}
		}
	}}/>

</div>