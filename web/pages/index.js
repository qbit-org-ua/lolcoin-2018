import React from 'react'

import Link from 'next/link'

import { QRCode } from 'react-qr-svg'

import { LOLCOINS_API_SERVER_URL } from '../components/constants'
import Layout from '../components/Layout'
import Loading from '../components/Loading'

class LOLcoinsTable extends React.Component {
  render() {
    return (
      <table className="table table-striped">
        <thead>
          <tr>
            <th>Имя</th>
            <th>Баланс ЛОЛкоинов</th>
            <th>Действие</th>
          </tr>
        </thead>
        <tbody>
          {this.props.users.map(([userId, user]) => (
            <tr key={userId}>
              <td>{user.full_name}</td>
              <td>
                <div>{user.balance}</div>
              </td>
              <td><Link href={{ pathname: '/transfers/new', query: { user_id: userId } }}><button className="btn btn-primary">Отправить ЛОЛкоины</button></Link></td>
            </tr>
          ))}
        </tbody>
      </table>
    )
  }
}

export default class Demo extends React.Component {
  state = { users: null }

  componentDidMount() {
    this.loadUsers()
  }

  render() {
    return (
      <Layout>
        <h1 className="pb-5 pt-5">ЛОЛкоин 2018</h1>
        {this.state.users === null ? <Loading /> : <LOLcoinsTable users={this.state.users} />}
      </Layout>
    )
  }

  loadUsers = () => {
    fetch(`${LOLCOINS_API_SERVER_URL}/users`)
      .then((response) => response.json())
      .then((users) => this.setState({ users: Object.entries(users).sort((left, right) => (left[1].full_name < right[1].full_name ? -1 : 1)) }))
  }
}
