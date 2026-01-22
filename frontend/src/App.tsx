import { useState, useEffect } from 'react'
import type { Accounting } from './types/accountings'

function App() {
  const [data, setData] = useState<Accounting[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [editingId, setEditingId] = useState<number | null>(null);

  const [formData, setFormData] = useState({
    name: '',
    income_expenditure: 0,
    price: 0,
    memo: '',
    created_by: 'admin',
    updated_by: 'admin'
  });

  const fetchData = async () => {
    try {
      const res = await fetch('http://127.0.0.1:8080/api/accountings');
      if (!res.ok) throw new Error(`Fetch Error: ${res.status}`);
      const json = await res.json();
      setData(json);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const isEdit = editingId !== null;
    const method = isEdit ? 'PUT' : 'POST';
    const url = isEdit 
      ? `http://127.0.0.1:8080/api/accountings/${editingId}`
      : 'http://127.0.0.1:8080/api/accountings';

    try {
      const res = await fetch(url, {
        method,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData),
      });
      if (!res.ok) throw new Error('Failed to save');
      
      setEditingId(null);
      setFormData({ ...formData, name: '', price: 0, memo: '' });
      fetchData();
    } catch (err: any) {
      alert(err.message);
    }
  };

  const handleDelete = async (id: number) => {
    if (!confirm('本当に削除しますか？')) return;
    try {
      const res = await fetch(`http://127.0.0.1:8080/api/accountings/${id}`, {
        method: 'DELETE',
      });
      if (!res.ok) throw new Error('Failed to delete');
      fetchData();
    } catch (err: any) {
      alert(err.message);
    }
  };

  const startEdit = (item: Accounting) => {
    setEditingId(item.accounting_id);
    setFormData({
      name: item.name,
      income_expenditure: item.income_expenditure,
      price: item.price,
      memo: item.memo || '',
      created_by: item.created_by,
      updated_by: 'admin'
    });
    window.scrollTo({ top: 0, behavior: 'smooth' });
  };

  return (
    <div style={{ padding: '20px', maxWidth: '800px', margin: '0 auto' }}>
      <h2>{editingId ? "編集モード" : "新規登録"}</h2>
      <form onSubmit={handleSubmit} style={{ padding: '20px', borderRadius: '8px' }}>
        <div>名称: <input type="text" value={formData.name} onChange={e => setFormData({...formData, name: e.target.value})} required /></div>
        <div>区分: 
          <select value={formData.income_expenditure} onChange={e => setFormData({...formData, income_expenditure: Number(e.target.value)})}>
            <option value={0}>支出</option>
            <option value={1}>収入</option>
          </select>
        </div>
        <div>金額: <input type="number" value={formData.price === 0 ? '' : formData.price} onChange={e => setFormData({...formData, price: Number(e.target.value)})} /></div>
        <div>備考: <input type="text" value={formData.memo} onChange={e => setFormData({...formData, memo: e.target.value})} /></div>
        <button type="submit" style={{ marginTop: '10px' }}>{editingId ? "更新" : "保存"}</button>
        {editingId && <button onClick={() => { setEditingId(null); setFormData({...formData, name: '', price: 0, memo: ''}) }}>中止</button>}
      </form>

      <hr />
      <h1>履歴一覧</h1>
      {loading ? <p>Loading...</p> : (
        <table border={1} style={{ width: '100%', textAlign: 'center' }}>
          <thead>
            <tr>
              <th>ID</th><th>区分</th><th>名称</th><th>金額</th><th>操作</th>
            </tr>
          </thead>
          <tbody>
            {data.map((item) => (
              <tr key={item.accounting_id}>
                <td>{item.accounting_id}</td>
                <td>{item.income_expenditure === 1 ? '💰' : '💸'}</td>
                <td>{item.name}</td>
                <td>¥{item.price.toLocaleString()}</td>
                <td>
                  <button onClick={() => startEdit(item)}>編集</button>
                  <button onClick={() => handleDelete(item.accounting_id)} style={{ color: 'red' }}>削除</button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  )
}

export default App
