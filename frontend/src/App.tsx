import { useEffect, useState } from 'react'

const API_URL = import.meta.env.VITE_API_URL ?? 'http://localhost:8080'

interface SystemInfo {
  hostname: string
  os_name: string
  kernel_version: string
  architecture: string
  uptime: number
}

interface DiskUsage {
  name: string
  mount_point: string
  total_space: number
  available_space: number
}

interface SystemResources {
  cpu_usage: number
  memory_total: number
  memory_used: number
  swap_total: number
  swap_used: number
  disks: DiskUsage[]
}

function formatBytes(bytes: number): string {
  const gb = bytes / 1024 ** 3
  return `${gb.toFixed(1)} GB`
}

function formatUptime(seconds: number): string {
  const hours = Math.floor(seconds / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  return `${hours}h ${minutes}m`
}

function App() {
  const [info, setInfo] = useState<SystemInfo | null>(null)
  const [resources, setResources] = useState<SystemResources | null>(null)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    Promise.all([
      fetch(`${API_URL}/api/system/info`).then((r) => r.json()),
      fetch(`${API_URL}/api/system/resources`).then((r) => r.json()),
    ])
      .then(([infoData, resourcesData]) => {
        setInfo(infoData)
        setResources(resourcesData)
      })
      .catch(() => setError('Failed to reach backend'))
  }, [])

  return (
    <div className="min-h-screen bg-neutral-950 text-neutral-100 p-8">
      <h1 className="text-2xl font-semibold mb-6">Nethrune Hub</h1>

      {error && <p className="text-red-400">{error}</p>}

      {info && (
        <div className="bg-neutral-900 rounded-lg p-4 mb-4">
          <h2 className="text-lg font-medium mb-2">System</h2>
          <p>Hostname: {info.hostname}</p>
          <p>OS: {info.os_name}</p>
          <p>Kernel: {info.kernel_version}</p>
          <p>Architecture: {info.architecture}</p>
          <p>Uptime: {formatUptime(info.uptime)}</p>
        </div>
      )}

      {resources && (
        <div className="bg-neutral-900 rounded-lg p-4">
          <h2 className="text-lg font-medium mb-2">Resources</h2>
          <p>CPU: {resources.cpu_usage.toFixed(1)}%</p>
          <p>
            Memory: {formatBytes(resources.memory_used)} / {formatBytes(resources.memory_total)}
          </p>
          <p>
            Swap: {formatBytes(resources.swap_used)} / {formatBytes(resources.swap_total)}
          </p>

          <h3 className="text-md font-medium mt-4 mb-2">Disks</h3>
          {resources.disks.map((disk) => (
            <p key={disk.name}>
              {disk.mount_point} ({disk.name}):{' '}
              {formatBytes(disk.total_space - disk.available_space)} / {formatBytes(disk.total_space)}
            </p>
          ))}
        </div>
      )}
    </div>
  )
}

export default App