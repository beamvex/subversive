import React from 'react';
import { Post } from '../types/Post';

export const Post: React.FC<{ post: Post }> = ({ post }) => {
  const handleVote = (vote: number) => {
    // Handle vote logic
  };

  return (
    <div className="bg-white p-4 rounded shadow mb-4">
      <h2 className="text-xl font-semibold">{post.title}</h2>
      <p className="text-gray-700 my-2">{post.content}</p>
      <div className="flex items-center space-x-2">
        <button
          onClick={() => handleVote(1)}
          className="text-green-600 hover:text-green-700"
        >
          ▲
        </button>
        <span className="text-gray-600">{post.votes}</span>
        <button
          onClick={() => handleVote(-1)}
          className="text-red-600 hover:text-red-700"
        >
          ▼
        </button>
      </div>
    </div>
  );
};
