import { Container, Input, Text, Button, Checkbox } from "@chakra-ui/react"
import { useEffect, useState } from "react"
import { invoke } from "@tauri-apps/api"

interface ITask {
	id: number
	title: string
	is_done: boolean
}

function App() {
	const [tasks, setTasks] = useState<ITask[]>([])
	const [taskName, setTaskName] = useState<string>("")
	const getTasks = async () => {
		const initialTasks = await invoke<ITask[]>("get_tasks")
		setTasks(initialTasks)
	}
	const createTask = (newTask: ITask) => {
		invoke("create_task", { newTask })
		getTasks()
	}

	useEffect(() => {
		getTasks()
	}, [])

	return (
		<Container as="main">
			<Text my="4">Todo app with Rust</Text>
			<Input
				placeholder="Enter task name..."
				onChange={(event) => setTaskName(event.target.value)}
			/>
			<Button
				type="submit"
				onClick={() => {
					createTask({
						title: taskName,
						id: Math.floor(Math.random() * 100),
						is_done: false,
					})
					setTaskName("")
				}}
			>
				Add task
			</Button>
			<Container>
				{tasks.length < 1 && <Text>Pas de tâches renseignées</Text>}
				{tasks.map((task: ITask, id: number) => (
					<ul key={id}>
						<Text>
							<Checkbox checked={task.is_done} /> {task.title}
						</Text>
					</ul>
				))}
			</Container>
		</Container>
	)
}

export default App
