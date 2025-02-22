import React, { useState } from 'react';
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { Header } from './components/Header';
import { PostList } from './components/PostList';
import { PostForm } from './components/PostForm';
import { Post } from './types/Post';

function App() {
  const [posts, setPosts] = useState<Post[]>([]);

  const handleCreatePost = (title: string, content: string) => {
    const newPost: Post = {
      id: crypto.randomUUID(),
      title,
      content,
      votes: 0,
      createdAt: new Date(),
    };
    setPosts((prevPosts) => [newPost, ...prevPosts]);
  };

  const handleVote = (postId: string, vote: number) => {
    setPosts((prevPosts) =>
      prevPosts.map((post) =>
        post.id === postId ? { ...post, votes: post.votes + vote } : post
      )
    );
  };

  return (
    <div className="min-h-screen bg-gray-100">
      <Header />
      <main className="container mx-auto p-4">
        <PostForm onCreatePost={handleCreatePost} />
        <PostList posts={posts} onVote={handleVote} />
      </main>
    </div>
  );
}

export default App;
