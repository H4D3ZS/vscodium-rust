import requests
import json
import time
from typing import Dict, Any, Optional

class AntigravityClient:
    def __init__(self, host: str = "127.0.0.1", port: int = 1536):
        self.base_url = f"http://{host}:{port}"
        
    def spawn_subagent(self, task: str) -> str:
        """
        Spawn an autonomous subagent in the background.
        Returns the Task ID of the spawned agent.
        """
        payload = {"task": task}
        headers = {"Content-Type": "application/json"}
        
        try:
            response = requests.post(f"{self.base_url}/api/subagent", json=payload, headers=headers)
            response.raise_for_status()
            data = response.json()
            task_id = data.get("task_id", "")
            print(f"🚀 Subagent spawned successfully: {task_id}")
            print(f"   Output will be saved to: subagent_{task_id}_result.md")
            return task_id
        except Exception as e:
            print(f"🔴 Failed to spawn subagent: {e}")
            raise

    def get_status(self) -> Dict[str, Any]:
        """
        Fetch the global orchestrator status (e.g. active subagents).
        """
        try:
            response = requests.get(f"{self.base_url}/api/subagents/status")
            response.raise_for_status()
            return response.json()
        except Exception as e:
            print(f"🔴 Failed to fetch orchestrator status: {e}")
            return {"active_subagents": 0}

if __name__ == "__main__":
    client = AntigravityClient()
    print("--- Antigravity God Protocol SDK ---")
    status = client.get_status()
    print(f"Active Background Subagents: {status.get('active_subagents', 0)}")
    
    # Example usage:
    # client.spawn_subagent("Write a Python script to calculate Fibonacci sequence.")
