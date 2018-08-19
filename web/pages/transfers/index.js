import { LOLCOINS_API_SERVER_URL } from '../../components/constants'
import Layout from '../../components/Layout'
import Loading from '../../components/Loading'

function secondsToDateString(seconds) {
  const d = new Date(0)
  d.setUTCSeconds(seconds)
  return d.toLocaleString()
}

class TransfersTable extends React.Component {
  render() {
    const { users, transfers } = this.props
    return (
      <table className="table table-striped">
        <thead>
          <tr>
            <th>{`Дата`}</th>
            <th>{`Отправитель`}</th>
            <th>{`Получатель`}</th>
            <th>{`Количество ЛОЛкоинов`}</th>
          </tr>
        </thead>
        <tbody>
          {transfers.map(transfer => (
            <tr key={transfer.date.secs_since_epoch}>
              <td>{secondsToDateString(transfer.date.secs_since_epoch)}</td>
              <td>{users[transfer.from].full_name}</td>
              <td>{users[transfer.to].full_name}</td>
              <td>{transfer.amount}</td>
            </tr>
          ))}
        </tbody>
      </table>
    )
  }
}

export default class extends React.Component {
  state = { users: null, transfers: null }

  componentDidMount() {
    this.loadUsers()
    this.loadTransfers()
  }

  render() {
    return (
      <Layout>
        <h1 className="pt-5 pb-5">{`Все транзации ЛОЛкоинов`}</h1>
        {(this.state.users === null || this.state.transfers === null) ? <Loading /> : <TransfersTable users={this.state.users} transfers={this.state.transfers} />}
      </Layout>
    )
  }

  loadUsers = () => {
    fetch(`${LOLCOINS_API_SERVER_URL}/users`)
      .then(response => response.json())
      .then(users => this.setState({ users }))
  }

  loadTransfers = () => {
    fetch(`${LOLCOINS_API_SERVER_URL}/transfers`)
      .then(response => response.json())
      .then(transfers => this.setState({ transfers }))
  }
}
