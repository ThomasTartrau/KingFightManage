function timestampToDate(timestamp: string) {
  return new Date(timestamp).toLocaleDateString('fr-FR', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  })
}

export default {
  timestampToDate,
}